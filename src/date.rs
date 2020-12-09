use chrono::prelude::*;
use askama::Template;

#[derive(Template, Clone, PartialEq, Eq)]
#[template(path = "date.html")]
pub struct Date(NaiveDate, Option<Role>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Role { Published, Revised }

use Role::*;
impl Date {
    pub fn new(inner: NaiveDate) -> Self {
        Date(inner, None)
    }
    pub fn published(inner: NaiveDate) -> Self {
        Date(inner, Some(Published))
    }
    pub fn revised(inner: NaiveDate) -> Self {
        Date(inner, Some(Revised))
    }
    pub fn iso8601(&self) -> String {
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

impl From<NaiveDate> for Date {
    fn from(nd: NaiveDate) -> Self { Date::new(nd) }
}

#[delegate(self.0)]
impl Date {
    pub fn year(&self) -> i32;
    pub fn month(&self) -> u32;
    pub fn month0(&self) -> u32;
    pub fn day(&self) -> u32;
    pub fn day0(&self) -> u32;
    pub fn weekday(&self) -> Weekday;
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}

impl PartialOrd<Date> for Date {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) }
}
