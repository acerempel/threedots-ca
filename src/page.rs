use relative_path::RelativePathBuf;
use askama::Template;
use crate::article::Article;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Page<'a, T: PageContent> {
    pub output_path: RelativePathBuf,
    pub content: &'a T,
    pub header: &'a [Article],
    pub footer: &'a [Article],
}

pub trait PageContent: std::fmt::Display {
    fn title(&self) -> Option<&str>;
    fn description(&self) -> Option<&str>;
    fn url(&self) -> &str;
    fn canonical(&self) -> Option<&str>;
}