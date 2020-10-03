use crate::link::Link;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Page<'a, T: PageContent> {
    pub content: &'a T,
    pub header: &'a [Link<'a>],
    pub footer: &'a [Link<'a>],
}

pub trait PageContent: std::fmt::Display {
    fn title(&self) -> Option<&str>;
    fn description(&self) -> Option<&str>;
    fn url(&self) -> &str;
    fn canonical(&self) -> Option<&str>;
    fn link(&self) -> Link {
        Link {
            content: self.title().unwrap(),
            description: self.description(),
            url: &self.url(),
        }
    }
}