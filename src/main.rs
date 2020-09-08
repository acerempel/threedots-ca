#[macro_use]
extern crate lazy_static;

use tera::Tera;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*.njk") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

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

fn output_path(input_path: &Path) -> Option<PathBuf> {
    let mut result = input_path.to_path_buf();
    let input_ext = input_path.extension();
    if input_ext == Some(OsStr::new("md")) || input_ext == Some(OsStr::new("html")) {
        result.pop();
        result.push(input_path.file_stem()?);
        result.push("index.html");
    };
    Some(result)
}

fn main() {
    for entry in WalkDir::new("content")
                     .into_iter()
                     .filter_entry(|e| !is_hidden(e))
                     .filter_map(|e| e.ok())
                     .filter(|e| is_file(e)) {
        println!("{}", entry.path().display());
        if entry.path().extension() == Some(OsStr::new("md")) {
            convert_markdown(entry.path());
        };
    };
}
