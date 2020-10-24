use serde::Deserialize;
use askama::Template;
use std::collections::HashSet;

use crate::date::Date;
use crate::URL;
use crate::link::Link;
use crate::prose::FromProse;
use crate::page::PageContent;

#[derive(Template)]
#[template(path = "post.html")]
pub struct Post {
    pub date: Date,
    pub date_revised: Option<Date>,
    url: URL,
    title: Option<String>,
    pub content: String,
    description: Option<String>,
    synopsis: Option<String>,
    canonical: Option<String>,
    pub tags: HashSet<String>,
} 

impl Post {
    pub fn summary(&self) -> Summary {
        let content = // TODO what about excerpt
            if let Some(synopsis) = &self.synopsis { SummaryContent::Synopsis(&synopsis) }
            else if let Some(excerpt) = &self.content.rsplit("<!-- FOLD -->").nth(1) {
                SummaryContent::Excerpt(excerpt)
            }
            else { SummaryContent::FullContent(&self.content) };
        Summary {
            date: &self.date,
            url: &self.url,
            title: self.title.as_deref(),
            description: self.description.as_deref(),
            content
        }
    }
}

#[derive(Deserialize)]
pub struct Data {
    date: chrono::NaiveDate,
    #[serde(default)] date_revised: Option<chrono::NaiveDate>,
    #[serde(default)] title: Option<String>,
    #[serde(default)] description: Option<String>,
    #[serde(default)] synopsis: Option<String>,
    #[serde(default)] canonical: Option<String>,
    #[serde(default)] tags: HashSet<String>,
}

impl FromProse for Post {
    type FrontMatter = Data;
    fn from_prose(front_matter: Self::FrontMatter, content: String,
        url: String) -> Post {
        Post {
            content, canonical: front_matter.canonical, date: Date(front_matter.date),
            date_revised: front_matter.date_revised.map(Date),
            title: front_matter.title, description: front_matter.description, synopsis: front_matter.synopsis,
            url, tags: front_matter.tags
        }
    }
}

impl PageContent for Post {
    fn url(&self) -> &str { &self.url }
    fn title(&self) -> Option<&str> { self.title.as_deref() }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn canonical(&self) -> Option<&str> { self.canonical.as_deref() }
}

#[derive(Template)]
#[template(path = "post-summary.html")]
pub struct Summary<'a> {
    date: &'a Date,
    url: &'a str,
    title: Option<&'a str>,
    description: Option<&'a str>,
    pub content: SummaryContent<'a>,
}

pub enum SummaryContent<'a> {
    Synopsis(&'a str),
    Excerpt(&'a str),
    FullContent(&'a str),
}

impl<'a> Summary<'a> {
    pub fn link(&'a self) -> Option<Link<'a>> {
        self.title.map(|title| Link::new(self.url, title).description_opt(self.description))
    }
}
