use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pimisi {
    #[serde(default = "Pimisi::default_input_dir")]
    pub input_dir: String,
    #[serde(default = "Pimisi::default_output_dir")]
    pub output_dir: String,
}

impl Pimisi {

    fn default_output_dir() -> String { String::from("_site") }
    fn default_input_dir() -> String { String::from("content") }

}
