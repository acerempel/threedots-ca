use crate::nav::Nav;
use crate::Output;
use serde::Deserialize;
use askama::Template;

use crate::date::Date;
use crate::ContentKind;
use crate::URL;
use crate::path::NominalPath;
use crate::link::Link;

#[derive(Template)]
#[template(path = "post.html", print = "code")]
struct Full <'a> {
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
    fn full(&'a self, nav: Nav<'a>) -> Full<'a> {
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
    fn summary(&'a self) -> Summary<'a> {
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
struct PostData<'a> {
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

#[derive(Template)]
#[template(path = "post-summary.html")]
struct Summary<'a> {
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