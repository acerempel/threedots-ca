use serde::Deserialize;
use askama::Template;

use crate::date::Date;
use crate::ContentKind;
use crate::URL;
use crate::path::{NominalPath, Input};
use crate::link::Link;

#[derive(Template)]
#[template(path = "post.html", print = "code")]
struct Full <'a> {
    date: Date,
    date_revised: Option<Date>,
    title: Option<&'a str>,
    content: &'a str,
    description: Option<&'a str>,
}

struct Details <'a> {
    date: Date,
    date_revised: Option<Date>,
    input_path: NominalPath<Input>,
    title: Option<&'a str>,
    content: &'a str,
    content_kind: ContentKind,
    description: Option<&'a str>,
    synopsis: Option<&'a str>,
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
}

#[derive(Template)]
#[template(path = "post-summary.html")]
struct Summary<'a> {
    date: Date,
    url: URL,
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
    fn link(&self) -> Option<Link<'a>> {
        self.title.map(|title| Link { content: title, url: self.url.clone(), description: self.description /* TODO */ })
    }
}