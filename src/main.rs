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

///! Turn *.html into */index.html, likewise with *.md.
fn get_output_path(input_path: &Path) -> Option<PathBuf> {
    let mut result = PathBuf::from("_site");
    result.push(input_path);
    let input_ext = input_path.extension();
    if input_ext == Some(OsStr::new("md")) || input_ext == Some(OsStr::new("html")) {
        result.pop();
        result.push(input_path.file_stem()?);
        result.push("index.html");
    };
    Some(result)
}

use std::fs;
use std::io;
use pulldown_cmark::{Parser, html};

fn convert_markdown(input_path: &Path, output_path: &Path) -> Result<(), io::Error>{
    let input = fs::read_to_string(input_path)?;
    let parser = Parser::new(&input);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);

    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `get_output_path`.
    for parent in output_path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; };

    fs::write(output_path, output_buf)?;
    Ok(())
}

fn main() {
    for entry in WalkDir::new("content")
                     .into_iter()
                     .filter_entry(|e| !is_hidden(e))
                     .filter_map(|e| e.ok())
                     .filter(|e| is_file(e))
    {
        let output_path = get_output_path(entry.path()).expect("Invalid file name");
        if entry.path().extension() == Some(OsStr::new("md")) {
            match convert_markdown(entry.path(), &output_path) {
                Ok(()) => { println!("{}: converted to {}", entry.path().display(), output_path.display()) },
                Err(err) => { println!("{}: error occured converting to {}, namely {}",
                                       entry.path().display(),
                                       output_path.display(),
                                       err) }
            }
        } else { println!("{}: copying to {}", entry.path().display(), output_path.display()) };
    };
}