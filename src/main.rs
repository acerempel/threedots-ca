#[macro_use]
extern crate anyhow;

use crate::post::Post;
use relative_path::{RelativePath, RelativePathBuf};
use anyhow::Result;

use serde_json::value::Value;

/// We use `Map` because that's what Handlebars uses under the hood
/// (by way of serde_json).
type Metadata = serde_json::map::Map<String,Value>;

mod kind;
mod post;
mod date;
mod link;
mod page;
mod prose;

use kind::*;
use prose::FromProse;

use std::path::Path;

mod path;
use path::*;

use std::fs;

/// Look at a file path and figure out, based on the file
/// extension(s) or lack thereof, how we should treat it.
fn discern_file_kind(input_path: &RelativePath) -> Result<FileKind> {
    let input_filename = input_path.file_name().expect("No filename!!");
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

use pulldown_cmark::{Parser, html};

/// Read a file, separate from the content and parse a YAML metadata
/// block if there is one, and return both metadata and content.
fn read_file_with_front_matter<P: FromProse>(input_path: &Path) -> Result<(P::FrontMatter, String)> {
    let entire_content = fs::read_to_string(input_path)?;
    if let Some(front_plus_content) = entire_content.strip_prefix("---") {
        // We have a YAML metadata block. Split the block from the
        // content that follows.
        let mut parts = front_plus_content.splitn(2, "---");
        let front_matter: P::FrontMatter = serde_yaml::from_str(parts.next().unwrap())?;

        // If somehow the file begins with "---", and has YAML we can
        // parse, but no closing "---", then that's fine, we just say
        // that the content is the empty string.
        let content = parts.next().unwrap_or("").to_string();
        Ok((front_matter, content))
    } else {
        Err(anyhow!("No YAML!"))
    }
}

type Html = String;

// Turn some markdown into HTML. This is a trivial wrapper around
// pulldown-cmark's API.
fn render_markdown(input: String) -> Html {
    let parser = Parser::new(&input);
    // We can reasonably estimate that the length of the HTML will be at
    // least as great of the length of the markdown, so maybe we can
    // skip a few allocations by allocating that much up front.
    let mut output_buf = String::with_capacity(input.len());
    html::push_html(&mut output_buf, parser);
    output_buf
}

// Write some HTML to a file, creating the parent directories of the
// file if they don't already exist.
fn write_page(output_path: &Path, content: Html) -> Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    create_parent_directories(&output_path)?;
    fs::write(output_path, content)?;
    Ok(())
}

use std::fs::File;

mod configuration;
use configuration::{Pimisi, SortDirection};

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
    let mut posts: Vec<Post> = Vec::with_capacity(64);
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
                let output_path = output_path_relative.to_path(pimisi.output_dir);
                create_parent_directories(&output_path)?;
                fs::copy(input_path, output_path)?; Ok(())
            },
            FileKind::Content(content_kind, output_path, url) => {
                println!("{}: reading content", input_path_nominal);
                match input_path_nominal.parent() {
                    Some(p) if p == "posts" => {
                        let (front_matter, raw_content) = read_file_with_front_matter::<Post>(&input_path)?;
                        let content = match content_kind {
                            // TODO escaping of e.g. '&' surrounded by whitespace?
                            ContentKind::Html => raw_content,
                            ContentKind::Markdown => render_markdown(raw_content),
                        };
                        let post = Post::from_prose(front_matter, content, url);
                        posts.push(post); Ok(()) },
                    Some(_) => todo!(),
                    None => panic!("nonsensical path: {}", input_path_nominal)
                }
            }
        }
    })?; // }}}

    // APPLY TEMPLATES {{{
    for mut page in pages.into_iter() {
        // Render it!
        let output = templates.render(&template_name, &page.data)?;
        let output_path_nominal = page.output_path;
        println!("{}: writing to {}", page.input_path, output_path_nominal);
        let output_path_real = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
        write_page(output_path_real, output)?;
    } // }}}
    Ok(())
}
