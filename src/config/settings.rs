use {
    crate::config::args::Args,
    clap::Parser,
    serde::Deserialize,
    std::{ffi::CString, fs::File, io::BufReader},
};

fn default_interactive() -> bool {
    false
}

#[derive(Deserialize)]
pub struct Settings {
    pub utils_path: CString,

    #[serde(default = "default_interactive")]
    pub interactive: bool,
}

impl Default for Settings {
    fn default() -> Self {
        serde_json::from_reader::<BufReader<File>, Self>(BufReader::new(
            File::open(Args::parse().config_file).expect("Failed to open config file"),
        ))
        .expect("Failed to read config file")
    }
}
