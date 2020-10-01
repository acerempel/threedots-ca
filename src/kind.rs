use relative_path::{RelativePath,RelativePathBuf};
use anyhow::Result;

/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
pub enum FileKind {
    Content(ContentKind, RelativePathBuf, URL),
    Asset(RelativePathBuf),
}

pub type URL = String;

/// Concerning a file that has page content in it, what format that
/// content is in.
pub enum ContentKind {
    Markdown,
    Html,
}

/// Look at a file path and figure out, based on the file
/// extension(s) or lack thereof, how we should treat it.
pub fn discern_file_kind(input_path: &RelativePath) -> Result<FileKind> {
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
        let content_url = || if stem == "index" {
            input_parent_dir
                .and_then(|p| if p == "" { None } else { Some(p) })
                .map(|dir| format!("/{}/", dir))
                .unwrap_or(String::from("/")) }
            else { input_parent_dir
                .and_then(|p| if p == "" { None } else { Some(p) })
                .map(|dir| format!("/{}/{}/", dir, stem))
                .unwrap_or_else(|| format!("/{}/", stem)) };
        match input_ext {
            "md" => Ok( FileKind::Content(ContentKind::Markdown, index_html(), content_url()) ),
            "html" => Ok( FileKind::Content(ContentKind::Html, index_html(), content_url()) ),
            _ => Ok( FileKind::Asset(input_path.to_owned()) ),
        }
    } else { Ok( FileKind::Asset(input_path.to_owned()) ) }
}

