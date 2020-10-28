use serde::Deserialize;
use askama::Template;
use std::collections::HashSet;
use chrono::NaiveDate;

use crate::date::Date;
use crate::URL;
use crate::link::Link;
use crate::prose::FromProse;
use crate::page::PageContent;

#[derive(Deserialize)]
pub struct Data {
    #[serde(default)] date: Option<NaiveDate>,
    #[serde(default)] date_revised: Option<NaiveDate>,
    title: String,
    #[serde(default)] description: Option<String>,
    #[serde(default)] canonical: Option<String>,
    #[serde(default)] link_text: Option<String>,
    #[serde(default)] weight: i32,
    #[serde(default)] tags: HashSet<String>
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct Article {
    date: Option<Date>,
    date_revised: Option<Date>,
    url: URL,
    title: String,
    content: String,
    link_text: Option<String>,
    pub weight: i32,
    description: Option<String>,
    canonical: Option<String>,
    tags: HashSet<String>,
} 

impl Article {
    pub fn link(&self) -> Link {
        let link_text = self.link_text.as_ref().unwrap_or(&self.title);
        Link::new(&self.url, link_text)
            .description_opt(self.description.as_deref())
            .date_revised_opt(self.date_revised.as_ref())
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

impl PageContent for Article {
    fn url(&self) -> &str { &self.url }
    fn title(&self) -> Option<&str> { Some(&self.title) }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn canonical(&self) -> Option<&str> { self.canonical.as_deref() }
}

impl FromProse for Article {
    type FrontMatter = Data;
    fn from_prose(front_matter: Self::FrontMatter, content: String,
        url: String) -> Article {
        Article {
            content, canonical: front_matter.canonical, date: front_matter.date.map(Date::published),
            date_revised: front_matter.date_revised.map(Date::revised),
            title: front_matter.title, description: front_matter.description,
            url, weight: front_matter.weight, tags: front_matter.tags,
            link_text: front_matter.link_text
        }
    }
}

