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

struct Pimisi {
    input_dir: String,
    output_dir: String,
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

impl Pimisi {

    /// Look at a file path and figure out, based on the file
    /// extension(s) or lack thereof, how we should treat it.
    fn discern_file_kind(&self, input_path: &Path) -> Result<FileKind> {
        // Easier to work with if we have a string. `Path` has not very
        // many methods defined on it.
        let input_path_str = input_path.to_str()
                .map(|s| Ok(s)).unwrap_or_else(|| Err(anyhow!("Filename not unicode: {:?}", input_path)))?;

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

    fn prepend_output_dir(&self, path: &Path) -> PathBuf {
        let output_dir: &Path = self.output_dir.as_ref();
        output_dir.join(path)
    }

    /// Given the path (without the input directory) to an file that has
    /// page content, and whether it is markdown or HTML, compute the
    /// path that we must write its corresponding output to. This
    /// involves turning \*.html into \*/index.html (unless the filename
    /// is *already* index.html), likewise with \*.md.
    fn content_output_path(&self, input_path: &Path, input_kind: ContentKind) -> Result<PathBuf> {
        let mut result = self.prepend_output_dir(input_path);
        // Is the closure here a Haskell-ism?
        let mut to_index_html = || -> Result<(),anyhow::Error> {
            result.pop();
            let input_stem = input_path.file_stem()
                // I think the error case here is impossible, because a
                // path without a file stem would be one that ends in a
                // slash (I think?), but we don't get such paths from the directory walking.
                .map(|s| Ok(s)).unwrap_or_else(|| Err(anyhow!("Weird input path: {:?}", input_path)))?;
            result.push(input_stem); result.push("index.html");
            Ok(())
        };
        match input_kind {
            ContentKind::Markdown => { to_index_html()?; },
            ContentKind::Html => {
                if input_path.file_name() == Some("index.html".as_ref()) {
                    return Ok(result);
                } else { to_index_html()?; }
            },
        };
        Ok(result)
    }

}

use std::fs;
use pulldown_cmark::{Parser, html};

/// Read a file, separate from the content and parse a YAML metadata
/// block if there is one, and return both metadata and content.
fn read_file_with_front_matter(input_path: &Path) -> Result<(Metadata, String)> {
    let entire_content = fs::read_to_string(input_path)?;
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

struct Html(String);

struct Content {
    metadata: Metadata,
    input_path: PathBuf,
    content: Html,
}

// Turn some markdown into HTML. This is a trivial wrapper around
// pulldown-cmark's API.
fn render_markdown(input: String) -> Html {
    let parser = Parser::new(&input);
    let mut output_buf = String::new(); // I guess we should maybe give a capacity hint
    html::push_html(&mut output_buf, parser);
    Html(output_buf)
}

// Write some HTML to a file, creating the parent directories of the
// file if they don't already exist.
fn write_page(output_path: &Path, content: Html) -> Result<()> {
    // The path may, in principle, have no parent; this is impossible here because we prepend the
    // output directory in `output_path`.
    for parent in output_path.parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; };
    fs::write(output_path, content.0)?;
    Ok(())
}

fn register_tag_for_page<'a>(tags: &mut BTreeMap<String, Vec<&'a Content>>, page: &'a Content, t: &str) {
    match tags.get_mut(t) {
        Some(v) => v.push(page),
        None => { tags.insert(t.to_owned(), vec![page]); () },
    };
}

use handlebars::Handlebars;

fn main() -> Result<()> {
    let pimisi = Pimisi { output_dir: String::from("_site")
                        , input_dir: String::from("content")
                        , template_suffix: String::from(".tpl") };

    let mut pages: Vec<Content> = Vec::with_capacity(32);
    let mut tags: BTreeMap<String, Vec<&Content>> = BTreeMap::new();
    let mut templates = Handlebars::new();
    templates.set_strict_mode(true);
    for entry in WalkDir::new(&pimisi.input_dir).into_iter()
                .filter_entry(|e| !is_hidden(e)) // Filter out hidden files (.\*)
                .filter_map(|e| e.ok()) // Ignore any errors produced by walkdir
                .filter(|e| is_file(e)) // Skip directories and whatever else is not a file (symbolic links too I guess)
    {
        // The real path, for doing IO with.
        let input_path_real = entry.path();
        // The path with the input directory stripped, for making
        // available as a variable in templates, and for computing the
        // URL and output path with.
        let input_path_nominal =
            // I don't think `strip_prefix` is quite this smart.
            if pimisi.input_dir == "." { input_path_real }
            else { input_path_real.strip_prefix(&pimisi.input_dir)? };

        let file_kind = pimisi.discern_file_kind(input_path_nominal)?;

        // I would prefer eventually to not bail on the first
        // error, but print the errors with a count and process all the
        // files we can, also counting them.
        match file_kind {
            FileKind::Asset => {
                let output_path = pimisi.prepend_output_dir(input_path_nominal);
                fs::copy(input_path_real, output_path)?; ()
            },
            FileKind::Template { name } => {
                templates.register_template_file(&name, input_path_nominal)?;
            },
            FileKind::Content(content_kind) => {
                let (metadata, content) = read_file_with_front_matter(input_path_real)?;
                let hypertext = match content_kind {
                    ContentKind::Html => Html(content),
                    ContentKind::Markdown => render_markdown(content),
                };

                // Remember to put this somewhere else
                // let output_path = pimisi.content_output_path(entry.path(), content_kind)?;

                let page = Content {
                    metadata,
                    content: hypertext,
                    input_path: input_path_nominal.to_owned() };
                pages.push(page);
            }
        }
    };

    for page in pages.iter() {
        // Each page is tagged with the name of its parent directory.
        let dir_tag = page.input_path.parent()
            .and_then(|p| p.file_name())
            // TODO Log something upon decoding failure!
            .and_then(|p| p.to_str());
        dir_tag.map(|t| register_tag_for_page(&mut tags, page, t));

        if let Some(Value::Sequence(meta_tags)) = page.metadata.get("tags") {
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
    };
    Ok(())
}
