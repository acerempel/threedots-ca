#[macro_use]
extern crate anyhow;

use std::collections::HashMap;
use crate::post::Post;
use relative_path::{RelativePathBuf, RelativePath};
use anyhow::Result;

mod kind;
mod post;
mod date;
mod link;
mod page;
mod prose;
mod article;
mod all_posts;

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

fn url_to_path(url: &str) -> RelativePathBuf {
    // Strip leading slash
    let mut path = RelativePath::new(&url[1..]).to_owned();
    if path.ends_with("/") { path.push("index.html"); };
    path
}

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
    let mut posts: Vec<Post> = Vec::with_capacity(64);
    let mut top_nav: Vec<Article> = Vec::with_capacity(8);
    let footer_nav: Vec<Article> = Vec::new();
    let mut misc: Vec<Article> = Vec::with_capacity(8);
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
            FileKind::Content(content_kind, url) => {
                println!("{}: reading content", input_path_nominal);
                match input_path_nominal.parent() {
                    Some(p) if p == "posts" => {
                        let post = read_prose::<Post>(input_path, content_kind, url)?;
                        posts.push(post); Ok(()) },
                    Some(_) => {
                        let article = read_prose::<Article>(input_path, content_kind, url)?;
                        if article.has_tag("top_nav") { top_nav.push(article); }
                        else if article.has_tag("misc_list") { misc.push(article); }
                        else { println!("{}: dangling article!", input_path_nominal) };
                        Ok(()) },
                    None => panic!("nonsensical path: {}", input_path_nominal)
                }
            }
        }
    })?; // }}}

    posts.sort_unstable_by(|p1, p2| p2.date.0.cmp(&p1.date.0));
    top_nav.sort_unstable_by_key(|a| a.weight);

    use page::{Page, PageContent};
    use askama::Template;
    use std::io::Write;
    use chrono::Datelike;

    fn render_page_to_file<P: PageContent>(page: Page<P>, pimisi: &Pimisi) -> Result<()> {
        let output_path = url_to_path(page.content.url()).to_path(&pimisi.output_dir);
        println!("Writing url {} to path {}", page.content.url(), output_path.display());
        let mut output_file = File::create(output_path)?;
        let rendered = page.render()?;
        let _ = output_file.write(rendered.as_ref())?; Ok(())
    }

    for post in posts.iter() {
        let page = Page {
            header: &top_nav[..], footer: &footer_nav[..],
            content: post };
        render_page_to_file(page, &pimisi)?;
    }

    let mut posts_by_year: HashMap<i32, Vec<post::Summary>> = HashMap::new();
    for post in posts.iter() {
        posts_by_year.entry(post.date.0.year()).or_insert_with(|| {
            let mut v = Vec::with_capacity(5);
            v.push(post.summary()); v
        });
    };
    use all_posts::AllPosts;
    let all_posts_page = Page {
        header: &top_nav[..], footer: &footer_nav[..],
        content: &AllPosts { posts_by_year }
    };
    render_page_to_file(all_posts_page, &pimisi)?;

    Ok(())
}
