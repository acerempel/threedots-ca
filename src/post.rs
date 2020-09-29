use crate::nav::Nav;
use crate::Output;
use serde::Deserialize;
use askama::Template;

use crate::date::Date;
use crate::ContentKind;
use crate::URL;
use crate::path::NominalPath;
use crate::link::Link;
use crate::prose::FromProse;

#[derive(Template)]
#[template(path = "post.html")]
pub struct Full <'a> {
    date: &'a Date,
    date_revised: Option<&'a Date>,
    url: &'a str,
    title: Option<&'a str>,
    content: &'a str,
    description: Option<&'a str>,
    canonical: Option<&'a str>,
    nav: Nav<'a>
}

pub struct Post <'a> {
    date: Date,
    date_revised: Option<Date>,
    url: URL,
    output_path: NominalPath<Output>,
    title: Option<&'a str>,
    content: &'a str,
    content_kind: ContentKind,
    description: Option<&'a str>,
    synopsis: Option<&'a str>,
    canonical: Option<&'a str>,
} 

impl<'a> Post<'a> {
    pub fn full(&'a self, nav: Nav<'a>) -> Full<'a> {
        Full {
            date: &self.date,
            date_revised: self.date_revised.as_ref(),
            url: &self.url,
            title: self.title,
            content: self.content,
            description: self.description,
            canonical: self.canonical,
            nav
        }
    }
    pub fn summary(&'a self) -> Summary<'a> {
        let content =
            if let Some(synopsis) = self.synopsis { SummaryContent::Synopsis(synopsis) }
            else { SummaryContent::FullContent(self.content) };
        Summary {
            date: &self.date,
            url: &self.url,
            title: self.title,
            description: self.description,
            content
        }
    }
}

#[derive(Deserialize)]
pub struct Data<'a> {
    date: Date,
    #[serde(default)]
    date_revised: Option<Date>,
    #[serde(default)]
    title: Option<&'a str>,
    #[serde(default)]
    description: Option<&'a str>,
    #[serde(default)]
    synopsis: Option<&'a str>,
    #[serde(default)]
    canonical: Option<&'a str>,
}

impl<'a> FromProse<'a> for Post<'a> {
    type FrontMatter = Data<'a>;
    fn from_prose(front_matter: Self::FrontMatter, content: &'a str,
        content_kind: ContentKind, url: String,
        output_path: NominalPath<Output>) -> Post {
        Post {
            content, canonical: front_matter.canonical, date: front_matter.date,
            date_revised: front_matter.date_revised,
            title: front_matter.title, description: front_matter.description, synopsis: front_matter.synopsis,
            content_kind, url, output_path
        }
    }
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