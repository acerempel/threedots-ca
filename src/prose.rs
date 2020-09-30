use serde::de::DeserializeOwned;

pub trait FromProse {
    type FrontMatter: DeserializeOwned;
    fn from_prose(
        front_matter: Self::FrontMatter,
        content: String,
        url: String,
    ) -> Self;
}