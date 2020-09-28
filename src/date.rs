use serde::Deserializer;
use serde::de::Visitor;
use serde::Deserialize;
#[allow(unused)]
use chrono::NaiveDate;
use askama::Template;

#[derive(Template)]
#[template(path = "date.html")]
pub struct Date {
    year: u32,
    month: u8,
    day: u8,
    month_name: &'static str
}

struct DateVisitor;

impl<'de> Deserialize<'de> for Date {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        todo!()
    }
}

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn visit_str<E: serde::de::Error>(self, string: &str) -> Result<Date, E> {
        todo!()
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
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
