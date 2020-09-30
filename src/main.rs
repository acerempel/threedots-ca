#[macro_use]
extern crate anyhow;

use crate::post::Post;
use relative_path::{RelativePath, RelativePathBuf};
use anyhow::Result;

mod kind;
mod post;
mod date;
mod link;
mod page;
mod prose;

use kind::*;
use prose::read_prose;

use std::path::Path;

mod path;
use path::*;

use std::fs;

/// Look at a file path and figure out, based on the file
/// extension(s) or lack thereof, how we should treat it.
fn discern_file_kind(input_path: &RelativePath) -> Result<FileKind> {
    let input_ext_opt = input_path.extension();
    let input_stem = input_path.file_stem();
    if let Some(stem) = input_stem {
        let input_ext = input_ext_opt.unwrap();
        let input_parent_dir = input_path.parent();
        let index_html = || {
            if stem == "index" {
                input_parent_dir
                    .map(|dir| dir.join(RelativePath::new("index.html")))
                    .unwrap_or(RelativePath::new("index.html").to_relative_path_buf())
            } else {
                input_parent_dir
                    .map(|dir| dir.join(stem).join(RelativePath::new("index.html")))
                    .unwrap_or_else(|| RelativePath::new(stem).join(RelativePath::new("index.html")))
            }
        };
        let content_url = || input_parent_dir
            .map(|dir| format!("/{}/{}/", dir, stem))
            .unwrap_or_else(|| format!("/{}/", stem));
        match input_ext {
            "md" => Ok( FileKind::Content(ContentKind::Markdown, index_html(), content_url()) ),
            "html" => Ok( FileKind::Content(ContentKind::Html, index_html(), content_url()) ),
            _ => Ok( FileKind::Asset(input_path.to_owned()) ),
        }
    } else { Ok( FileKind::Asset(input_path.to_owned()) ) }
}

use std::fs::File;

mod configuration;
use configuration::Pimisi;

mod walk;

fn main() -> Result<()> {
    // INITIALIZE GLOBAL STATE AND CONFIGURATION {{{
    let config_file_path = "threedots.yaml";
    let pimisi: Pimisi = {
        // TODO what if the file does not exist? In that case defaults
        // should be used.
        let config_file = File::open(config_file_path)?;
        serde_yaml::from_reader(config_file)?
    };

    // let mut articles: Vec<Article> = Vec::with_capacity(32);
    let mut posts: Vec<(Post, RelativePathBuf)> = Vec::with_capacity(64);
    // }}}

    use walk::for_each_input_file;

    // WALK THE INPUT DIRECTORY {{{
    for_each_input_file(&Path::new(&pimisi.input_dir).join("posts"), |input_path| {
        // The path with the input directory stripped, for making
        // available as a variable in templates, and for computing the
        // URL and output path with.
        let input_path_nominal = strip_input_dir(&pimisi.input_dir, &input_path)?;

        let file_kind = discern_file_kind(&input_path_nominal)?;

        // INITIAL HANDLING OF INPUT FILES {{{
        // I would prefer eventually to not bail on the first
        // error, but print the errors with a count and process all the
        // files we can, also counting them.
        match file_kind {
            FileKind::Asset(output_path_relative) => {
                println!("{}: copying to {}", input_path_nominal, output_path_relative);
                let output_path = output_path_relative.to_path(&pimisi.output_dir);
                create_parent_directories(&output_path)?;
                fs::copy(input_path, output_path)?; Ok(())
            },
            FileKind::Content(content_kind, output_path, url) => {
                println!("{}: reading content", input_path_nominal);
                match input_path_nominal.parent() {
                    Some(p) if p == "posts" => {
                        let post = read_prose::<Post>(input_path, content_kind, url)?;
                        posts.push((post, output_path)); Ok(()) },
                    Some(_) => todo!(),
                    None => panic!("nonsensical path: {}", input_path_nominal)
                }
            }
        }
    })?; // }}}

    Ok(())
}
