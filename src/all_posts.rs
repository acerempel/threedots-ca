use askama::Template;
use std::collections::HashMap;
use crate::page::PageContent;
use crate::post;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct AllPosts<'a> {
    pub posts_by_year: HashMap<i32, Vec<post::Summary<'a>>>
}

impl<'a> PageContent for AllPosts<'a> {
    fn title(&self) -> Option<&str> { Some("Blog") }
    fn url(&self) -> &str { "/posts" }
    fn description(&self) -> Option<&str> { todo!() }
    fn canonical(&self) -> Option<&str> { None }
}