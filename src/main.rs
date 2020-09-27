#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde_json;

use anyhow::Result;

use std::collections::BTreeMap;
use serde_json::value::Value;

/// We use `Map` because that's what Handlebars uses under the hood
/// (by way of serde_json).
type Metadata = serde_json::map::Map<String,Value>;

/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
enum FileKind {
    Content(ContentKind, NominalPath<Output>, URL),
    Template { name: String },
    Asset(NominalPath<Output>),
}

type URL = String;

/// Concerning a file that has page content in it, what format that
/// content is in.
enum ContentKind {
    Markdown,
    Html,
}

use std::path::Path;

mod path;
use path::*;

use std::fs;

/// Look at a file path and figure out, based on the file
/// extension(s) or lack thereof, how we should treat it.
fn discern_file_kind(template_suffix: &str, input_path_nominal: &NominalPath<Input>) -> Result<FileKind> {
    let input_path = &input_path_nominal.path;
    let mut input_path_parts = input_path.rsplitn(2, '/');
    let input_filename = input_path_parts.next().expect("No filename!!");
    let input_parent_dir = input_path_parts.next();
    let same_input_path = || NominalPath::from(input_path.clone());
    let mut input_filename_parts = input_filename.rsplitn(2, '.');
    let input_ext_opt = input_filename_parts.next();
    let input_stem = input_filename_parts.next();
    if let Some(stem) = input_stem {
        let input_ext = input_ext_opt.unwrap();
        let index_html = || {
            let path = if stem == "index" {
                input_parent_dir
                    .map(|dir| [dir, "index.html"].join("/"))
                    .unwrap_or(String::from("index.html"))
            } else {
                input_parent_dir
                    .map(|dir| [dir, stem, "index.html"].join("/"))
                    .unwrap_or_else(|| [stem, "index.html"].join("/"))
            };
            NominalPath::from(path)
        };
        let content_url = || input_parent_dir
            .map(|dir| format!("/{}/{}/", dir, stem))
            .unwrap_or_else(|| format!("/{}/", stem));
        match input_ext {
            "md" => Ok( FileKind::Content(ContentKind::Markdown, index_html(), content_url()) ),
            "html" => Ok( FileKind::Content(ContentKind::Html, index_html(), content_url()) ),
            ext if ext == template_suffix => {
                let name = input_parent_dir.map(|dir| [dir, stem].join("/")).unwrap_or_else(|| stem.to_owned());
                Ok(FileKind::Template { name })
            },
            _ => Ok( FileKind::Asset(same_input_path()) ),
        }
    } else { Ok( FileKind::Asset(same_input_path()) ) }
}

use pulldown_cmark::{Parser, html};

/// Read a file, separate from the content and parse a YAML metadata
/// block if there is one, and return both metadata and content.
fn read_file_with_front_matter(input_path: &RealPath<Input>) -> Result<(Metadata, String)> {
    let entire_content = fs::read_to_string(&input_path)?;
    if let Some(front_plus_content) = entire_content.strip_prefix("---") {
        // We have a YAML metadata block. Split the block from the
        // content that follows.
        let mut parts = front_plus_content.splitn(2, "---");
        let metadata = serde_yaml::from_str(parts.next().unwrap())?;

        // If somehow the file begins with "---", and has YAML we can
        // parse, but no closing "---", then that's fine, we just say
        // that the content is the empty string.
        let content = parts.next().unwrap_or("").to_string();
        Ok((metadata, content))
    } else {
        // No YAML at the top. That's fine. No default metadata.
        let metadata = Metadata::new();
        let content = entire_content;
        Ok((metadata, content))
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
fn write_page(output_path: RealPath<Output>, content: Html) -> Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    create_parent_directories(&output_path)?;
    fs::write(output_path, content)?;
    Ok(())
}

fn determine_template_name(templates: &Handlebars, page: &Page) -> Option<String> {
    if let Some(Value::String(name)) = page.data.get("template") { Some(name.clone()) }
    else {
        let sans_ext = page.input_path.path.rsplitn(2, ".").nth(1).unwrap();
        if templates.has_template(&sans_ext) { Some(sans_ext.to_owned()) }
        else {
            // This is annoying; I just wanted to use `with_file_name`
            let dir_template_name = page.input_path.path
                .rsplitn(2, '/').nth(1) // Split on the last path separator, drop the filename
                .map(|n| [n, "/_each"].join(""))
                .unwrap_or(String::from("_each"));
            if templates.has_template(&dir_template_name) { Some(dir_template_name) }
            else { None }
        }
    }
}

/// Ready to be passed to Handlebars.
struct Page {
    input_path: NominalPath<Input>,
    output_path: NominalPath<Output>,
    data: Metadata
}

#[macro_use]
extern crate handlebars;
use handlebars::Handlebars;
use std::fs::File;

struct Tags(BTreeMap<String,Vec<Value>>);

impl Tags {
    fn new() -> Self {
        Tags(BTreeMap::new())
    }

    fn register(&mut self, t: &str, page: &Page){
        let val = Value::Object(page.data.clone());
        match self.0.get_mut(t) {
            Some(v) => v.push(val),
            None => { self.0.insert(t.to_owned(), vec![val]); () },
        };
        println!("{}: registering with tag {}", page.input_path, t);
    }

}

mod helpers;
use helpers::{ParseDate, take};

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

    let mut pages: Vec<Page> = Vec::with_capacity(32);
    let mut tags = Tags::new();

    let mut templates = Handlebars::new();
    templates.set_strict_mode(true);
    templates.register_helper("date_parse", Box::new(ParseDate));
    templates.register_helper("take", Box::new(take));
    // }}}

    use walk::for_each_input_file;

    // WALK THE INPUT DIRECTORY {{{
    for_each_input_file(&pimisi.input_dir, |path| {
        // The real path, for doing IO with.
        let input_path_real = real_input_path(path);

        // The path with the input directory stripped, for making
        // available as a variable in templates, and for computing the
        // URL and output path with.
        let input_path_nominal = strip_input_dir(&pimisi.input_dir, &input_path_real)?;

        let file_kind = discern_file_kind(&pimisi.template_suffix, &input_path_nominal)?;

        // INITIAL HANDLING OF INPUT FILES {{{
        // I would prefer eventually to not bail on the first
        // error, but print the errors with a count and process all the
        // files we can, also counting them.
        match file_kind {
            FileKind::Asset(output_path_nominal) => {
                println!("{}: copying to {}", input_path_nominal, output_path_nominal);
                let output_path = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
                create_parent_directories(&output_path)?;
                fs::copy(input_path_real, output_path)?; Ok(())
            },
            FileKind::Template { name } => {
                println!("{}: registering template as {}", input_path_nominal, name);
                templates.register_template_file(&name, input_path_real.path)?; Ok(())
            },
            FileKind::Content(content_kind, output_path, url) => {
                println!("{}: reading content", input_path_nominal);
                let (mut data, raw_content) = read_file_with_front_matter(&input_path_real)?;
                let content = match content_kind {
                    // TODO escaping of e.g. '&' surrounded by whitespace?
                    ContentKind::Html => raw_content,
                    ContentKind::Markdown => render_markdown(raw_content),
                };

                data.insert(String::from("content"), Value::String(content));
                data.insert(String::from("path"), json!({"input": input_path_nominal.path, "output": output_path.path}));
                data.insert(String::from("url"), Value::String(url));
                let page = Page { data, input_path: input_path_nominal, output_path };
                pages.push(page); Ok(())
            }
        } /* }}} */
    })?; // }}}

    // REGISTER THE TAGS {{{
    for page in pages.iter() {
        // Each page is tagged with (1) the name of its enclosing
        // directory, sans trailing slash, and with internal slashes
        // replaced by underscores;
        let page_input_path: &Path = page.input_path.as_ref();
        let dir_tag = page_input_path.parent()
            // TODO Log something upon decoding failure!
            .and_then(|p| p.to_str())
            .map(|p| p.replace("/", "_"))
            .map(|p| if p.is_empty() { String::from("_") } else { p });
        dir_tag.as_ref().map(|t| tags.register(t, page));

        // and with (2) each string value in an array in the "tags"
        // field in the file's front matter.
        if let Some(Value::Array(meta_tags)) = page.data.get("tags") {
            for tag in meta_tags.iter() {
                // Log non-string values as errors?
                if let Value::String(t) = tag {
                    // Refuse to register a page with the same tag
                    // twice!
                    match dir_tag { Some(ref d) if d != t => tags.register(t, page), _ => () }
                } else {
                    println!("{}: has non-string tag, namely {:?}", page.input_path, tag)
                }
            }
        }
    }; /* }}} */

    for (tag, sort_by) in pimisi.tags_sorting {
        let compare_pages = |page1: &Value, page2: &Value| {
            let comparison = match (page1.get(&sort_by.key), page2.get(&sort_by.key)) {
                    (Some(Value::String(s1)), Some(Value::String(s2))) =>
                        s1.cmp(&s2),
                    (Some(Value::Number(s1)), Some(Value::Number(s2))) =>
                        s1.as_f64().partial_cmp(&s2.as_f64())
                            .unwrap_or(std::cmp::Ordering::Equal),
                    (a, b) => {
                        println!("comparison keys {} are neither both strings nor both numbers, they are namely {:?} and {:?}", sort_by.key, a, b);
                        std::cmp::Ordering::Equal
                    }
            };
            match &sort_by.direction {
                SortDirection::Ascending => comparison,
                SortDirection::Descending => comparison.reverse()
            }
        };
        tags.0.get_mut(&tag).map(|t| t.sort_unstable_by(compare_pages));
    }

    // APPLY TEMPLATES {{{
    for mut page in pages.into_iter() {
        let template_name = determine_template_name(&templates, &page)
                .ok_or_else(|| anyhow!("{}: no template found!", page.input_path))?;
        println!("{}: applying template {}", page.input_path, template_name);

        // Make available a list of pages under each tag as a variable with the
        // name of the tag (slashes replaced with underscores).
        for (tag, tagged_pages) in tags.0.iter() {
            // Use of `clone()` here is unfortunate, but necessary because serde_json::Value
            // needs owned data. Would wish not to go via Value.
            page.data.insert(tag.clone(), Value::Array(tagged_pages.clone()));
        };

        // Render it!
        let output = templates.render(&template_name, &page.data)?;
        let output_path_nominal = page.output_path;
        println!("{}: writing to {}", page.input_path, output_path_nominal);
        let output_path_real = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
        write_page(output_path_real, output)?;
    } // }}}
    Ok(())
}
