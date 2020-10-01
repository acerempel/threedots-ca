use chrono::NaiveDate;
use askama::Template;

#[derive(Template, Clone)]
#[template(path = "date.html")]
pub struct Date(pub NaiveDate);

impl Date {
    fn iso8601(&self) -> String {
        format!("{}", self.0)
    }
    fn nice(&self) -> String {
        format!("{}", self.0.format("%B %-d, %Y"))
    }
}