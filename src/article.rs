use serde::Deserialize;
use askama::Template;

use crate::date::Date;
use crate::URL;
use crate::link::Link;
use crate::prose::FromProse;
use crate::page::PageContent;

#[derive(Deserialize)]
pub struct Data {
    #[serde(default)]
    date: Option<Date>,
    #[serde(default)]
    date_revised: Option<Date>,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    canonical: Option<String>,
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct Article {
    date: Option<Date>,
    date_revised: Option<Date>,
    url: URL,
    title: String,
    content: String,
    description: Option<String>,
    canonical: Option<String>,
} 

impl Article {
    pub fn link<'a>(&'a self) -> Link<'a> {
        Link { content: &self.title, description: self.description.as_deref(), url: &self.url }
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
            content, canonical: front_matter.canonical, date: front_matter.date,
            date_revised: front_matter.date_revised,
            title: front_matter.title, description: front_matter.description,
            url
        }
    }
}

