#[macro_use]
extern crate anyhow;

use crate::post::Post;
use relative_path::RelativePathBuf;
use anyhow::Result;

mod kind;
mod post;
mod date;
mod link;
mod page;
mod prose;
mod article;

use kind::*;
use prose::read_prose;

use std::path::Path;

mod path;
use path::*;

use std::fs;

use std::fs::File;

mod configuration;
use configuration::Pimisi;

mod walk;

use article::Article;

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
    let mut top_nav: Vec<(Article, RelativePathBuf)> = Vec::with_capacity(8);
    let mut misc: Vec<(Article, RelativePathBuf)> = Vec::with_capacity(8);
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
                    Some(_) => {
                        let article = read_prose::<Article>(input_path, content_kind, url)?;
                        if article.has_tag("top_nav") { top_nav.push((article, output_path)); }
                        else if article.has_tag("misc_list") { misc.push((article, output_path)); }
                        else { println!("{}: dangling article!", input_path_nominal) };
                        Ok(()) },
                    None => panic!("nonsensical path: {}", input_path_nominal)
                }
            }
        }
    })?; // }}}

    Ok(())
}
