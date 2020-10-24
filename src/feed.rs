use crate::link::Link;
use askama::Template;
use crate::post::Post;
use crate::post::SummaryContent;
use crate::page::PageContent;
use chrono::prelude::*;

#[derive(Template)]
#[template(path = "atom.xml")]
pub struct Feed<'a> {
    pub all_posts: &'a [Post],
    pub datetime_now: DateTime<Local>,
    pub site_title: String,
    pub site_desc: String,
    pub site_author: String,
    pub site_url: String,
    
}
const DESCRIPTION: &'static str = "Provide this link to your RSS reader, and get updates whenever something new is posted! (It is actually an Atom feed, but whatever.)";

impl<'a> Feed<'a> {
    pub fn link(&self) -> Link<'a> {
        Link::new("/posts/feed.xml", "RSS feed").description(DESCRIPTION)
    }
}

mod filters {
    use std::borrow::Cow;
    pub fn sans_tags<'a>(string: &'a str) -> askama::Result<Cow<'a, str>> {
        use regex::Regex;
        lazy_static!{
            static ref TAG_REGEX: Regex = Regex::new("<[^>]*>").unwrap();
        }
        Ok(TAG_REGEX.replace(string, ""))
    }
    pub fn first_few_words(string: &str) -> askama::Result<String> {
        let mut words = Vec::with_capacity(11);
        for word in string.split_whitespace().take(11) {
            words.push(word);
            if word.ends_with(&[',',':',';','–','.','?','!'][..]) { break };
        }
        Ok(words.join(" "))
    }
}
