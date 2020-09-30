use anyhow::Result;
use std::fs;
use std::path::Path;
use serde::de::DeserializeOwned;
use crate::kind::ContentKind;

pub trait FromProse {
    type FrontMatter: DeserializeOwned;
    fn from_prose(
        front_matter: Self::FrontMatter,
        content: String,
        url: String,
    ) -> Self;
}

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

pub fn read_prose<P: FromProse>(input_path: &Path, content_kind: ContentKind, url: String) -> Result<P> {
    let (front_matter, raw_content) = read_file_with_front_matter::<P>(&input_path)?;
    let content = match content_kind {
        // TODO escaping of e.g. '&' surrounded by whitespace?
        ContentKind::Html => raw_content,
        ContentKind::Markdown => render_markdown(raw_content),
    };
    Ok(P::from_prose(front_matter, content, url))
}

use pulldown_cmark::{Parser, html};

pub type Html = String;

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
