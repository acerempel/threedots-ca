use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pimisi {
    #[serde(default = "Pimisi::default_input_dir")]
    pub input_dir: String,
    #[serde(default = "Pimisi::default_output_dir")]
    pub output_dir: String,
    #[serde(default = "Pimisi::default_template_suffix")]
    pub template_suffix: String,
    #[serde(default, rename = "tags")]
    pub tags_sorting: BTreeMap<String, SortBy>,
}

#[derive(Deserialize)]
pub struct SortBy { pub key: String, pub direction: SortDirection }

pub enum SortDirection { Ascending, Descending }

use serde::Deserializer;

impl<'de> Deserialize<'de> for SortDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(SortDirectionVisitor)
    }
}

struct SortDirectionVisitor;

use std::fmt;

impl<'de> serde::de::Visitor<'de> for SortDirectionVisitor {
    type Value = SortDirection;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the string \"asc\", \"ascending\", \"desc\", or \"descending\"")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match v {
            "ascending" => Ok(SortDirection::Ascending),
            "asc" => Ok(SortDirection::Ascending),
            "descending" => Ok(SortDirection::Descending),
            "desc" => Ok(SortDirection::Descending),
            _ => Err(E::unknown_variant(v, &["asc", "ascending", "desc", "descending"]))
        }
    }
}

impl Pimisi {

    fn default_output_dir() -> String { String::from("_site") }
    fn default_input_dir() -> String { String::from("content") }
    fn default_template_suffix() -> String { String::from("hbs") }

}
