/// What kind of file? Does it contain content that we must process and
/// output; is it a template that we must load and let Tera take care of;
/// or is it an asset that we just copy over?
use relative_path::RelativePathBuf;

pub enum FileKind {
    Content(ContentKind, ContentFormat, RelativePathBuf, URL),
    Asset(RelativePathBuf),
}

pub type URL = String;

/// Concerning a file that has page content in it, what format that
/// content is in.
pub enum ContentKind {
    Markdown,
    Html,
}

pub enum ContentFormat {
    Homepage,
    Post,
    Article,
}