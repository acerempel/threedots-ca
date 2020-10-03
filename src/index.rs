use crate::link::Link;
use crate::article::Article;
use crate::post::Post;
use crate::page::PageContent;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub latest_posts: &'a [Post],
    pub misc_pages: &'a [Link<'a>],
    pub content: &'a Article,
}

impl<'a> PageContent for Index<'a> {
    fn title(&self) -> Option<&str> { self.content.title() }
    fn url(&self) -> &str { self.content.url() }
    fn description(&self) -> Option<&str> { self.content.description() }
    fn canonical(&self) -> Option<&str> { self.content.canonical() }
}