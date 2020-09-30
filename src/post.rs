use serde::Deserialize;
use askama::Template;

use crate::date::Date;
use crate::URL;
use crate::link::Link;
use crate::prose::FromProse;
use crate::page::PageContent;

#[derive(Template)]
#[template(path = "post.html")]
pub struct Post {
    date: Date,
    date_revised: Option<Date>,
    url: URL,
    title: Option<String>,
    content: String,
    description: Option<String>,
    synopsis: Option<String>,
    canonical: Option<String>,
} 

impl Post {
    pub fn summary<'a>(&'a self) -> Summary<'a> {
        let content = // TODO what about excerpt
            if let Some(synopsis) = &self.synopsis { SummaryContent::Synopsis(&synopsis) }
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
    date: Date,
    #[serde(default)]
    date_revised: Option<Date>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    synopsis: Option<String>,
    #[serde(default)]
    canonical: Option<String>,
}

impl FromProse for Post {
    type FrontMatter = Data;
    fn from_prose(front_matter: Self::FrontMatter, content: String,
        url: String) -> Post {
        Post {
            content, canonical: front_matter.canonical, date: front_matter.date,
            date_revised: front_matter.date_revised,
            title: front_matter.title, description: front_matter.description, synopsis: front_matter.synopsis,
            url
        }
    }
}

impl PageContent for Post {
    fn url(&self) -> &str { &self.url }
    fn title(&self) -> Option<&str> { todo!() }
    fn description(&self) -> Option<&str> { todo!() }
    fn canonical(&self) -> Option<&str> { todo!() }
}

#[derive(Template)]
#[template(path = "post-summary.html")]
pub struct Summary<'a> {
    date: &'a Date,
    url: &'a str,
    title: Option<&'a str>,
    description: Option<&'a str>,
    content: SummaryContent<'a>,
}

enum SummaryContent<'a> {
    Synopsis(&'a str),
    Excerpt(&'a str),
    FullContent(&'a str),
}

impl<'a> Summary<'a> {
    fn link(&'a self) -> Option<Link<'a>> {
        self.title.map(|title| Link { content: title, url: self.url, description: self.description })
    }
}