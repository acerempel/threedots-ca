use serde::Deserializer;
use serde::de::Visitor;
use serde::de;
use serde::Deserialize;
use chrono::NaiveDate;
use chrono::Datelike;
use askama::Template;

#[derive(Template, Clone)]
#[template(path = "date.html")]
pub struct Date {
    pub year: i32,
    month: u32,
    day: u32,
    month_name: &'static str
}

struct DateVisitor;

impl<'de> Deserialize<'de> for Date {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(DateVisitor)
    }
}

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn visit_str<E: serde::de::Error>(self, string: &str) -> Result<Date, E> {
        string.parse::<NaiveDate>()
            .map(|d| Date { month: d.month(), day: d.day(), year: d.year(), month_name: month_name(d.month()) })
            .map_err(|e| de::Error::custom(format!("{}", e)))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a date string in ISO8601 format")
    }
}

fn month_name(y: u32) -> &'static str {
    match y {
        1 => "January", 2 => "February", 3 => "March",
        4 => "April", 5 => "May", 6 => "June",
        7 => "July", 8 => "August", 9 => "September",
        10 => "October", 11 => "November", 12 => "December",
        _ => panic!("Invalid month number: {}", y)
    }
}
