/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
use crate::path::*;

pub enum FileKind {
    Content(ContentKind, NominalPath<Output>, URL),
    Asset(NominalPath<Output>),
}

pub type URL = String;

/// Concerning a file that has page content in it, what format that
/// content is in.
pub enum ContentKind {
    Markdown,
    Html,
}

