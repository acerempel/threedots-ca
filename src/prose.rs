use serde::Deserialize;
use crate::kind::ContentKind;
use crate::path::{NominalPath, Output};

pub trait FromProse<'a> {
    type FrontMatter: Deserialize<'a>;
    fn from_prose(
        front_matter: Self::FrontMatter,
        content: &'a str,
        kind: ContentKind,
        url: String,
        output_path: NominalPath<Output>,
    ) -> Self;
}