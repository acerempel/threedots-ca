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

use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use anyhow::Result;

struct Pimisi {
    input_dir: String,
    output_dir: String,
    template_dir: String
}

impl Pimisi {

    /// Turn *.html into */index.html, likewise with *.md.
    fn output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let mut result = PathBuf::from(self.output_dir.clone());
        result.push(input_path.strip_prefix(&self.input_dir).expect("Terrible error!")); // better error message please
        let input_ext = input_path.extension();
        if input_ext == Some(OsStr::new("md")) || input_ext == Some(OsStr::new("html")) {
            result.pop();
            let input_stem = input_path.file_stem()
                .map(|s| Ok(s))
                .unwrap_or_else(|| Err(WeirdInputPath(input_path.to_path_buf())))?;
            result.push(input_stem);
            result.push("index.html");
        };
        Ok(result)
    }

}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct WeirdInputPath(PathBuf);

impl fmt::Display for WeirdInputPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(fmt, "A weird input path: {:?}", self.0)?; Ok(())
    }
}

impl Error for WeirdInputPath {}

use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml::Value;

type Metadata = HashMap<String, Value>;

#[derive(Deserialize)]
struct File {
    metadata: Metadata,
    content: String,
    path: PathBuf,
}

use std::fs;
use std::io;
use pulldown_cmark::{Parser, html};

fn read_file(input_path: &Path, output_path: &Path) -> Result<File> {
    let entire_content = fs::read_to_string(input_path)?;
    let path = input_path.to_path_buf();
    if let Some(front_plus_content) = entire_content.strip_prefix("---") {
        let parts = front_plus_content.splitn(2, "---");
        let metadata = serde_yaml::from_str(parts.next().unwrap())?;
        let content = parts.next().unwrap_or("").to_string();
        Ok(File { metadata, content, path })
    } else {
        let metadata = HashMap::new();
        let content = entire_content;
        Ok(File { metadata, content, path })
    }
}

struct Template {
    name: String,
    content: String,
}

struct Html(String);

struct Page {
    metadata: Metadata,
    input_path: PathBuf,
    output_path: PathBuf,
    content: Html,
}

fn render_markdown(input: File) -> Result<Page> {
    let parser = Parser::new(&input.content);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);
    Ok(Page {
        metadata: input.metadata, content: Html(output_buf),
        input_path: input.path, output_path: 
    })
}

fn write_page(page: RenderedPage) -> io::Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    for parent in output_path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; };
    fs::write(output_path, output_buf)?;
}

fn main() {
    let pimisi = Pimisi { output_dir: String::from("_site")
                        , input_dir: String::from("content")
                        , template_dir: String::from("templates") };

    for entry in WalkDir::new(&pimisi.input_dir)
                     .into_iter()
                     .filter_entry(|e| !is_hidden(e))
                     .filter_map(|e| e.ok())
                     .filter(|e| is_file(e))
    {
        let output_path = pimisi.output_path(entry.path()).expect("Invalid file name");
        if entry.path().extension() == Some(OsStr::new("md")) {
            let page = parse_markdown(entry.path(), &output_path);
            {
                Ok(()) => {
                    println!("{}: converted to {}", entry.path().display(), output_path.display());
                },
                Err(err) => { println!("{}: error occured converting to {}, namely {}",
                                       entry.path().display(),
                                       output_path.display(),
                                       err) }
            }
        } else { println!("{}: copying to {}", entry.path().display(), output_path.display()) };
    };
}
