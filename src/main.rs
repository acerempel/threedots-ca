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

#[macro_use]
extern crate serde_json;

use std::path::{Path, PathBuf};
use anyhow::Result;

use std::collections::BTreeMap;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct Pimisi {
    #[serde(default = "Pimisi::default_input_dir")]
    input_dir: String,
    #[serde(default = "Pimisi::default_output_dir")]
    output_dir: String,
    #[serde(default = "Pimisi::default_template_suffix")]
    template_suffix: String,
}

struct NominalPath<T: PathOrientation>{ path: String, phantom: PhantomData<T> }
struct RealPath<T: PathOrientation>{ path: PathBuf, phantom: PhantomData<T> }

trait PathOrientation {}

struct Input;
struct Output;
impl PathOrientation for Input {}
impl PathOrientation for Output {}

impl Pimisi {

    fn default_output_dir() -> String { String::from("_site") }
    fn default_input_dir() -> String { String::from("content") }
    fn default_template_suffix() -> String { String::from("hbs") }

}

/// Look at a file path and figure out, based on the file
/// extension(s) or lack thereof, how we should treat it.
fn discern_file_kind(template_suffix: &str, input_path_nominal: &NominalPath<Input>) -> Result<FileKind> {
    let input_path = &input_path_nominal.path;
    let mut input_path_parts = input_path.rsplitn(2, '/');
    let input_filename = input_path_parts.next().expect("No filename!!");
    let input_parent_dir = input_path_parts.next();
    let same_input_path = || NominalPath { path: input_path.clone(), phantom: PhantomData };
    if input_filename == "index.html"
        { let url = input_parent_dir.map(|dir| format!("/{}/", dir)).unwrap_or_else(|| String::from("/"));
          Ok(FileKind::Content(ContentKind::Html, same_input_path(), url)) }
    else {
        let mut input_filename_parts = input_filename.rsplitn(2, '.');
        let input_ext_opt = input_filename_parts.next();
        let input_stem = input_filename_parts.next();
        if let Some(stem) = input_stem {
            let input_ext = input_ext_opt.unwrap();
            let index_html = || NominalPath { path: input_parent_dir.map(|dir| [dir, stem, "index.html"].join("/")).unwrap_or_else(|| [stem, "index.html"].join("/")), phantom: PhantomData };
            let content_url = || input_parent_dir.map(|dir| format!("/{}/{}/", dir, stem)).unwrap_or_else(|| format!("/{}/", stem));
            match input_ext {
                "md" => Ok( FileKind::Content(ContentKind::Markdown, index_html(), content_url()) ),
                "html" => Ok( FileKind::Content(ContentKind::Html, index_html(), content_url()) ),
                ext if ext == template_suffix => Ok(FileKind::Template { name: input_parent_dir.map(|dir| [dir, stem].join("/")).unwrap_or_else(|| stem.to_owned()) }),
                _ => Ok( FileKind::Asset(same_input_path()) ),
            }
        } else { Ok( FileKind::Asset(same_input_path()) ) }
    }
}

use std::marker::PhantomData;

fn real_input_path(input_path: &Path) -> RealPath<Input> {
    RealPath { path: input_path.to_path_buf(), phantom: PhantomData }
}

fn prepend_output_dir(output_dir: &Path, path: NominalPath<Output>) -> RealPath<Output> {
    RealPath{ path: output_dir.join(path.path), phantom: path.phantom }
}

fn strip_input_dir(input_dir: &str, input_path_real: &RealPath<Input>) -> Result<NominalPath<Input>> {
    // I don't think `strip_prefix` is quite this smart.
    let stripped =
        if input_dir == "." { &input_path_real.path }
        else { input_path_real.path.strip_prefix(input_dir)? };
    stripped.to_str()
        .map(|s| Ok(NominalPath { path: s.to_owned(), phantom: PhantomData }))
        .unwrap_or_else(|| Err(anyhow!("not unicode path! {:?}", stripped)))
}

use std::fs;

fn create_parent_directories(output: &RealPath<Output>) -> Result<()> {
    for parent in output.path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; }; Ok(())
}

use pulldown_cmark::{Parser, html};

/// Read a file, separate from the content and parse a YAML metadata
/// block if there is one, and return both metadata and content.
fn read_file_with_front_matter(input_path: &RealPath<Input>) -> Result<(Metadata, String)> {
    let entire_content = fs::read_to_string(&input_path.path)?;
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
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);
    output_buf
}

// Write some HTML to a file, creating the parent directories of the
// file if they don't already exist.
fn write_page(output_path: RealPath<Output>, content: Html) -> Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    for parent in output_path.path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; };
    fs::write(output_path.path, content)?;
    Ok(())
}

fn determine_template_name(templates: &Handlebars, page: &Page) -> Option<String> {
    if let Some(Value::String(name)) = page.data.get("template") { Some(name.clone()) }
    else if templates.has_template(&page.input_path.path) { Some(page.input_path.path.clone()) }
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

/// Ready to be passed to Handlebars.
struct Page {
    input_path: NominalPath<Input>,
    output_path: NominalPath<Output>,
    data: Metadata
}

use handlebars::{Handlebars, ScopedJson, RenderError};
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
        println!("{}: registering with tag {}", page.input_path.path, t);
    }

}

impl handlebars::HelperDef for Tags {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &handlebars::Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>
        ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let param = helper.param(0)
            .ok_or_else(|| RenderError::new("no parameter given!"))
            .and_then(|v| v.value().as_str().ok_or_else(|| RenderError::new("parameter is not a string!")))?;
        Ok(self.0.get(param).map(|tags| ScopedJson::Derived(Value::Array(tags.clone()))))
    }
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

    let mut pages: Vec<Page> = Vec::with_capacity(32);
    let mut tags = Tags::new();

    let mut templates = Handlebars::new();
    templates.set_strict_mode(true);
    // }}}

    // WALK THE INPUT DIRECTORY {{{
    for entry in WalkDir::new(&pimisi.input_dir).into_iter()
                .filter_entry(|e| !is_hidden(e)) // Filter out hidden files (.\*)
                .filter_map(|e| e.ok()) // Ignore any errors produced by walkdir
                .filter(|e| is_file(e)) // Skip directories and whatever else is not a file (symbolic links too I guess)
    {
        // The real path, for doing IO with.
        let input_path_real = real_input_path(entry.path());
        // The path with the input directory stripped, for making
        // available as a variable in templates, and for computing the
        // URL and output path with.
        let input_path_nominal = strip_input_dir(&pimisi.input_dir, &input_path_real)?;

        let file_kind = discern_file_kind(&pimisi.template_suffix, &input_path_nominal)?;

        /*** INITIAL HANDLING OF INPUT FILES {{{ ***/
        // I would prefer eventually to not bail on the first
        // error, but print the errors with a count and process all the
        // files we can, also counting them.
        match file_kind {
            FileKind::Asset(output_path_nominal) => {
                println!("{}: copying to {}", input_path_nominal.path, output_path_nominal.path);
                let output_path = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
                create_parent_directories(&output_path)?;
                fs::copy(input_path_real.path, output_path.path)?; ()
            },
            FileKind::Template { name } => {
                println!("{}: registering template as {}", input_path_nominal.path, name);
                templates.register_template_file(&name, input_path_real.path)?;
            },
            FileKind::Content(content_kind, output_path, url) => {
                println!("{}: reading content", input_path_nominal.path);
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
                pages.push(page);
            }
        } /* }}} */
    }; // }}}

    // REGISTER THE TAGS {{{
    for page in pages.iter() {
        // Each page is tagged with the name of its parent directory.
        let page_input_path: &Path = page.input_path.path.as_ref();
        let dir_tag = page_input_path.parent()
            .and_then(|p| p.file_name())
            // TODO Log something upon decoding failure!
            .and_then(|p| p.to_str());
        dir_tag.map(|t| tags.register(t, page));

        if let Some(Value::Array(meta_tags)) = page.data.get("tags") {
            for tag in meta_tags.iter() {
                // Log non-string values as errors?
                match tag {
                    // Don't register with the same tag twice!
                    Value::String(t) if dir_tag != Some(t) =>
                        tags.register(t, page),
                    _ => ()
                }
            }
        }
    }; /* }}} */

    templates.register_helper("tag", Box::new(tags));

    for page in pages.into_iter() {
        let template_name = determine_template_name(&templates, &page);
        let output = match template_name {
            Some(name) => {
                println!("{}: applying template {}", page.input_path.path, name);
                templates.render(&name, &page.data)? },
            None => String::from("No content!"), // TODO do something better here
        };
        let output_path_nominal = page.output_path;
        println!("{}: writing to {}", page.input_path.path, output_path_nominal.path);
        let output_path_real = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
        write_page(output_path_real, output)?;
    }
    Ok(())
}
