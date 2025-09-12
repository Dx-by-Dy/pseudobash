use {
    crate::config::args::Args,
    clap::Parser,
    serde::Deserialize,
    std::{fs::File, io::BufReader},
};

#[derive(Deserialize)]
pub struct Settings {
    pub utils_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        serde_json::from_reader::<BufReader<File>, Self>(BufReader::new(
            File::open(Args::parse().config_file).expect("Failed to open config file"),
        ))
        .expect("Failed to read config file")
    }
}
