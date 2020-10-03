use askama::Template;
use std::collections::BTreeMap;
use crate::page::PageContent;
use crate::post;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct AllPosts<'a> {
    pub posts_by_year: BTreeMap<i32, Vec<post::Summary<'a>>>
}

impl<'a> PageContent for AllPosts<'a> {
    fn title(&self) -> Option<&str> { Some("Blog") }
    fn url(&self) -> &str { "/posts/" }
    fn description(&self) -> Option<&str> { Some("All posts in the three dots blog, grouped by year.") }
    fn canonical(&self) -> Option<&str> { None }
}