use askama::Template;
use crate::date::Date;

#[derive(Template)]
#[template(path = "link.html")]
pub struct Link<'a> {
    pub content: &'a str,
    pub description: Option<&'a str>,
    pub url: &'a str,
    pub date_revised: Option<&'a Date>,
}

impl<'a> Link<'a> {
    pub fn new(url: &'a str, content: &'a str) -> Self {
        Link { url, content, description: None, date_revised: None }
    }

    pub fn description(self, descr: &'a str) -> Self {
        Link { description: Some(descr), ..self }
    }

    pub fn description_opt(self, descr: Option<&'a str>) -> Self {
        Link { description: descr, ..self }
    }

    pub fn date_revised(self, date: &'a Date) -> Self {
        Link { date_revised: Some(date), ..self }
    }

    pub fn date_revised_opt(self, date: Option<&'a Date>) -> Self {
        Link { date_revised: date, ..self }
    }
}
