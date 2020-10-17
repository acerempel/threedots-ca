use serde::Deserialize;
use argh::FromArgs;

/// The raw configuration: each field is derived either from the command line
/// or from the configuration file.
#[derive(Deserialize, FromArgs)]
#[argh(description = "the static website builder for threedots.ca")]
pub struct Zimisi {
    /// where input files are to be found.
    #[serde(default)]
    #[argh(option, short = 'i')]
    pub input_dir: Option<String>,

    /// where to put the output files.
    #[serde(default)]
    #[argh(option, short = 'o')]
    pub output_dir: Option<String>,
}

pub struct Pimisi {
    pub input_dir: String,
    pub output_dir: String,
}

impl Pimisi {
    pub fn from_zimisis(command_line: Zimisi, config_file: Zimisi) -> Self {
        Pimisi {
            input_dir: command_line.input_dir
                .or(config_file.input_dir)
                .unwrap_or_else(Pimisi::default_input_dir),
            output_dir: command_line.output_dir
                .or(config_file.output_dir)
                .unwrap_or_else(Pimisi::default_output_dir),
        }
    }
    fn default_output_dir() -> String { String::from("_site") }
    fn default_input_dir() -> String { String::from("content") }
}
