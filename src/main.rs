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

/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
enum FileKind {
    Content(ContentKind),
    Template { name: String },
    Asset,
}

/// Concerning a file that has page content in it, what format that
/// content is in.
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

    /// Look at a file path and figure out, based on the file
    /// extension(s) or lack thereof, how we should treat it.
    fn discern_file_kind(&self, input_path: &Path) -> Result<FileKind> {
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

    /// Given the input path for a file that we are just copying over to
    /// the output directory, figure out what path we must copy it to. A
    /// trivial computation.
    fn asset_output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let mut result = PathBuf::from(self.output_dir.clone());
        result.push(input_path.strip_prefix(&self.input_dir)?);
        Ok(result)
    }

    /// Given the path to an file that has page content, and whether it
    /// is markdown or HTML, compute the path that we must write its
    /// corresponding output to. This involves turning \*.html into
    /// \*/index.html (unless the filename is *already* index.html),
    /// likewise with \*.md.
    fn content_output_path(&self, input_path: &Path, input_kind: ContentKind) -> Result<PathBuf> {
        let mut result = self.asset_output_path(input_path)?;
        // Is the closure here a Haskell-ism?
        let mut to_index_html = || -> Result<(),anyhow::Error> {
            result.pop();
            let input_stem = input_path.file_stem()
                // I think the error case here is impossible, because a
                // path without a file stem would be one that ends in a
                // slash (I think?), but we don't get such paths from the directory
                // walking.
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

/// Read a file, separate from the content and parse a YAML metadata
/// block if there is one, and return both metadata and content.
fn read_file(input_path: &Path) -> Result<LoadedFile> {
    let entire_content = fs::read_to_string(input_path)?;
    if let Some(front_plus_content) = entire_content.strip_prefix("---") {
        // We have a YAML metadata block. Split the block from the
        // content that follows.
        let mut parts = front_plus_content.splitn(2, "---");
        let metadata = serde_yaml::from_str(parts.next().unwrap())?;

        // If somehow the file begins with "---", and has YAML we can
        // parse, but no closing "---", then that's fine, we just say
        // that the content is the empty string.
        let content = parts.next().unwrap_or("").to_string();
        Ok(LoadedFile { metadata, content })
    } else {
        // No YAML at the top. That's fine. No default metadata.
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

// Turn some markdown into HTML. This is a trivial wrapper around
// pulldown-cmark's API.
fn render_markdown(input: String) -> Html {
    let parser = Parser::new(&input);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);
    Html(output_buf)
}

// Write some HTML to a file, creating the parent directories of the
// file if they don't already exist.
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
                .filter_entry(|e| !is_hidden(e)) // Filter out hidden files (.\*)
                .filter_map(|e| e.ok()) // Ignore any errors produced by walkdir
                .filter(|e| is_file(e)) // Skip directories and whatever else is not a file (symbolic links too I guess)
    {
        let file_kind = pimisi.discern_file_kind(entry.path())?;
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
