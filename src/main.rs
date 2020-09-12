#[macro_use]
extern crate lazy_static;

use tera::Tera;

use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.metadata().map(|e| e.is_file()).unwrap_or(false)
}

#[macro_use]
extern crate anyhow;

use std::path::{Path, PathBuf};
use anyhow::Result;

use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml::Value;

type Metadata = HashMap<String, Value>;

enum FileKind {
    Content(ContentKind),
    Template { name: String },
    Asset,
}

enum ContentKind {
    Markdown,
    Html,
}

struct LoadedFile {
    metadata: Metadata,
    content: String,
}

struct Pimisi {
    input_dir: String,
    output_dir: String,
    template_suffix: String,
}

impl Pimisi {

    fn examine_path(&self, input_path: &Path) -> Result<FileKind> {
        let kind = match input_path.extension().and_then(|e| e.to_str()) {
            Some("md") => FileKind::Content(ContentKind::Markdown),
            Some("html") => {
                let opt_name = input_path.file_stem()
                        .expect("No file stem!") // Not possible, since we filter out directories when walking.
                        .to_str().map(|s| Ok(s))
                        .unwrap_or_else(|| Err(anyhow!("Filename not unicode: {:?}", input_path)))?;
            match opt_name.strip_suffix(&self.template_suffix) {
                Some(name) => FileKind::Template { name: name.to_owned() },
                None => FileKind::Content(ContentKind::Html)
            }},
            _ => FileKind::Asset,
        };
        Ok(kind)
    }

    fn asset_output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let mut result = PathBuf::from(self.output_dir.clone());
        result.push(input_path.strip_prefix(&self.input_dir)?);
        Ok(result)
    }

    /// Turn *.html into */index.html, likewise with *.md.
    fn content_output_path(&self, input_path: &Path, input_kind: ContentKind) -> Result<PathBuf> {
        let mut result = self.asset_output_path(input_path)?;
        let mut to_index_html = || -> Result<(),anyhow::Error> {
            result.pop();
            let input_stem = input_path.file_stem()
                .map(|s| Ok(s)).unwrap_or_else(|| Err(anyhow!("Weird input path: {:?}", input_path)))?;
            result.push(input_stem); result.push("index.html");
            Ok(())
        };
        match input_kind {
            ContentKind::Markdown => { to_index_html()?; },
            ContentKind::Html => {
                if input_path.file_name() == Some("index.html".as_ref()) {
                    return Ok(result);
                } else { to_index_html()?; }
            },
        };
        Ok(result)
    }

}

use std::fs;
use pulldown_cmark::{Parser, html};

fn read_file(input_path: &Path) -> Result<LoadedFile> {
    let entire_content = fs::read_to_string(input_path)?;
    if let Some(front_plus_content) = entire_content.strip_prefix("---") {
        let mut parts = front_plus_content.splitn(2, "---");
        let metadata = serde_yaml::from_str(parts.next().unwrap())?;
        let content = parts.next().unwrap_or("").to_string();
        Ok(LoadedFile { metadata, content })
    } else {
        let metadata = HashMap::new();
        let content = entire_content;
        Ok(LoadedFile { metadata, content })
    }
}

struct Html(String);

struct Page {
    metadata: Metadata,
    input_path: PathBuf,
    output_path: PathBuf,
    content: Html,
}

fn render_markdown(input: String) -> Html {
    let parser = Parser::new(&input);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);
    Html(output_buf)
}

fn write_page(output_path: &Path, content: Html) -> Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    for parent in output_path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; };
    fs::write(output_path, content.0)?;
    Ok(())
}

fn main() -> Result<()> {
    let pimisi = Pimisi { output_dir: String::from("_site")
                        , input_dir: String::from("content")
                        , template_suffix: String::from(".tpl") };

    let mut pages = Vec::with_capacity(8);
    let mut templates = Vec::with_capacity(4);
    for entry in WalkDir::new(&pimisi.input_dir) .into_iter()
                     .filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok())
                     .filter(|e| is_file(e))
    {
        let file_kind = pimisi.examine_path(entry.path())?;
        match file_kind {
            FileKind::Asset => {
                let output_path = pimisi.asset_output_path(entry.path())?;
                fs::copy(entry.path(), output_path)?; ()
            },
            FileKind::Template { name } => {
                let loaded_file = read_file(entry.path())?;
                templates.push(loaded_file);
            },
            FileKind::Content(content_kind) => {
                let loaded_file = read_file(entry.path())?;
                let hypertext = match content_kind {
                    ContentKind::Html => Html(loaded_file.content),
                    ContentKind::Markdown => render_markdown(loaded_file.content),
                };
                let output_path = pimisi.content_output_path(entry.path(), content_kind)?;
                let page = Page {
                    content: hypertext, input_path: entry.path().to_owned(),
                    output_path, metadata: loaded_file.metadata };
                pages.push(page);
            }
        }
    };
    Ok(())
}
