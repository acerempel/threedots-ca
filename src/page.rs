use relative_path::RelativePathBuf;
use askama::Template;
use crate::link::Link;

pub struct Page<'a, T: PageContent> {
    pub output_path: RelativePathBuf,
    pub content: &'a T,
    pub header: &'a [Link<'a>],
    pub footer: &'a [Link<'a>],
}

pub trait PageContent: Template {
    fn title(&self) -> Option<&str>;
    fn description(&self) -> Option<&str>;
    fn url(&self) -> &str;
}