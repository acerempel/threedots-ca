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

use std::path::{Path, PathBuf};
use anyhow::Result;

use std::collections::BTreeMap;
use serde::Deserialize;
use serde_yaml::Value;

/// We use `BTreeMap` because that's what Handlebars uses under the hood
/// (by way of serde_json).
type Metadata = BTreeMap<String, Value>;

/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
enum FileKind {
    Content(ContentKind),
    Template { name: String },
    Asset,
}

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

/// Does it have such an extension? See `strip_extension` for why it is
/// terrible.
fn has_extension(path: &str, ext: &str) -> bool {
    strip_extension(path, ext).is_some()
}

/// This requires `ext` to have a leading '.'. Also the path separator
/// is hardcoded as a '/'. Terrible.
fn strip_extension<'a>(path: &'a str, ext: &str) -> Option<&'a str> {
    if let Some(stripped) = path.strip_suffix(ext) {
        // TODO this should be the platform path separator
        if stripped.ends_with('/') { return None; };
        if stripped.is_empty() { return None; };
        return Some(stripped);
    } else { return None; };
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
    fn default_template_suffix() -> String { String::from(".hbs") }

    /// Look at a file path and figure out, based on the file
    /// extension(s) or lack thereof, how we should treat it.
    fn discern_file_kind(&self, input_path: &NominalPath<Input>) -> Result<FileKind> {
        let input_path_str = &input_path.path;
        // What is the file extension? TODO This should handle XML, and
        // maybe be extensible … hmmm. I just do not like this bit of
        // code.
        let kind =
            if has_extension(input_path_str, ".md") {
                FileKind::Content(ContentKind::Markdown)
            } else if let Some(sans_ext) = strip_extension(input_path_str, &self.template_suffix) {
                FileKind::Template { name: sans_ext.to_owned() }
            } else if has_extension(input_path_str, ".html") {
                FileKind::Content(ContentKind::Html)
            } else { FileKind::Asset };
        Ok(kind)
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

/// Given the path (without the input directory) to an file that has
/// page content, and whether it is markdown or HTML, compute the
/// path that we must write its corresponding output to. This
/// involves turning \*.html into \*/index.html (unless the filename
/// is *already* index.html), likewise with \*.md.
fn content_output_path(input_path_nominal: NominalPath<Input>) -> Result<NominalPath<Output>> {
    let input_path = input_path_nominal.path;
    let mut input_path_parts = input_path.rsplitn(2, '/');
    let input_filename = input_path_parts.next().expect("No filename!!");
    if input_filename == "index.html"
        { Ok(NominalPath { path: input_path, phantom: PhantomData }) }
    else {
        let input_parent_dir = input_path_parts.next();
        let mut input_filename_parts = input_filename.rsplitn(2, '.');
        let input_ext = input_filename_parts.next();
        let input_stem = input_filename_parts.next();
        match input_stem {
            Some(stem) if input_ext == Some("md") || input_ext == Some("html") => {
                let path = [input_parent_dir.unwrap_or(""), stem, "/index.html"].join("");
                Ok(NominalPath { path, phantom: PhantomData }) },
            _ => Ok(NominalPath { path: input_path, phantom: PhantomData }),
        }
    }
}

fn asset_output_path(input_path: NominalPath<Input>) -> NominalPath<Output> {
    NominalPath { path: input_path.path, phantom: PhantomData }
}

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

fn register_tag_for_page<'a>(tags: &mut BTreeMap<String, Vec<&'a PageForTemplate>>, page: &'a PageForTemplate, t: &str) {
    match tags.get_mut(t) {
        Some(v) => v.push(page),
        None => { tags.insert(t.to_owned(), vec![page]); () },
    };
}

fn determine_template_name(templates: &Handlebars, page: &PageForTemplate) -> Option<String> {
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
struct PageForTemplate {
    input_path: NominalPath<Input>,
    data: BTreeMap<String,Value>
}

use handlebars::Handlebars;
use std::fs::File;

fn main() -> Result<()> {
    // INITIALIZE GLOBAL STATE AND CONFIGURATION {{{
    let config_file_path = "threedots.yaml";
    let pimisi: Pimisi = {
        // TODO what if the file does not exist? In that case defaults
        // should be used.
        let config_file = File::open(config_file_path)?;
        serde_yaml::from_reader(config_file)?
    };

    let mut pages: Vec<PageForTemplate> = Vec::with_capacity(32);
    let mut tags: BTreeMap<String, Vec<&PageForTemplate>> = BTreeMap::new();

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

        let file_kind = pimisi.discern_file_kind(&input_path_nominal)?;

        /*** INITIAL HANDLING OF INPUT FILES {{{ ***/
        // I would prefer eventually to not bail on the first
        // error, but print the errors with a count and process all the
        // files we can, also counting them.
        match file_kind {
            FileKind::Asset => {
                let output_path = prepend_output_dir(pimisi.output_dir.as_ref(), asset_output_path(input_path_nominal));
                // TODO this is repeated
                create_parent_directories(&output_path)?;
                fs::copy(input_path_real.path, output_path.path)?; ()
            },
            FileKind::Template { name } => {
                templates.register_template_file(&name, input_path_real.path)?;
            },
            FileKind::Content(content_kind) => {
                let (mut data, content) = read_file_with_front_matter(&input_path_real)?;
                let hypertext = match content_kind {
                    // TODO escaping of e.g. '&' surrounded by whitespace?
                    ContentKind::Html => content,
                    ContentKind::Markdown => render_markdown(content),
                };

                data.insert(String::from("content"), Value::String(hypertext));

                let page = PageForTemplate { data, input_path: input_path_nominal };
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
        dir_tag.map(|t| register_tag_for_page(&mut tags, page, t));

        if let Some(Value::Sequence(meta_tags)) = page.data.get("tags") {
            for tag in meta_tags.iter() {
                // Log non-string values as errors?
                match tag {
                    // Don't register with the same tag twice!
                    Value::String(t) if dir_tag != Some(t) =>
                        register_tag_for_page(&mut tags, page, t),
                    _ => ()
                }
            }
        }
    }; /* }}} */

    for page in pages.into_iter() {
        let template_name = determine_template_name(&templates, &page);
        let output = match template_name {
            Some(name) => templates.render(&name, &page.data)?,
            None => String::from("No content!"), // TODO do something better here
        };
        let output_path_nominal = content_output_path(page.input_path)?;
        let output_path_real = prepend_output_dir(pimisi.output_dir.as_ref(), output_path_nominal);
        write_page(output_path_real, output)?;
    }
    Ok(())
}
