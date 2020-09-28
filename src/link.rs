use askama::Template;
use crate::URL;

#[derive(Template)]
#[template(path = "link.html")]
pub struct Link<'a> {
    pub content: &'a str,
    pub description: Option<&'a str>,
    pub url: URL
}