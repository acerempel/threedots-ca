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

struct Pimisi {
    // TODO: these should be `String`s -- it implements AsRef<Path> and suchlike
    input_dir: String,
    output_dir: String,
    template_dir: String
}

impl Pimisi {

    ///! Turn *.html into */index.html, likewise with *.md.
    fn output_path(&self, input_path: &Path) -> Option<PathBuf> {
        let mut result = PathBuf::from(self.output_dir.clone());
        result.push(input_path.strip_prefix(&self.input_dir).expect("Terrible error!"));
        let input_ext = input_path.extension();
        if input_ext == Some(OsStr::new("md")) || input_ext == Some(OsStr::new("html")) {
            result.pop();
            result.push(input_path.file_stem()?);
            result.push("index.html");
        };
        Some(result)
    }

}

struct ParsedPage {
    data: (),
    content: String,
    input_path: PathBuf,
    output_path: PathBuf
}

use std::fs;
use std::io;
use pulldown_cmark::{Parser, html};

fn read_page(input_path: &Path, output_path: &Path) -> io::Result<ParsedPage> {
    let input = fs::read_to_string(input_path)?;

}

fn render_markdown(page: ParsedPage) -> io::Result<RenderedPage> {
    let parser = Parser::new(&input);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);

    Ok(())
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
