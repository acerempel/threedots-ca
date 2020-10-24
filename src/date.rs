use chrono::prelude::*;
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
    pub fn to_local_datetime(&self) -> DateTime<Local> {
        let with_time = self.0.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0));
        let zoned_datetime = Local.from_local_datetime(&with_time);
        zoned_datetime.latest().unwrap()
    }
}