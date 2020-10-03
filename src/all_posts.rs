use askama::Template;
use crate::page::PageContent;
use crate::post::Post;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct AllPosts<'a> {
    pub posts_by_year: Vec<(i32, &'a [Post])>
}

impl<'a> PageContent for AllPosts<'a> {
    fn title(&self) -> Option<&str> { Some("Blog") }
    fn url(&self) -> &str { "/posts/" }
    fn description(&self) -> Option<&str> { Some("All posts in the three dots blog, grouped by year.") }
    fn canonical(&self) -> Option<&str> { None }
}