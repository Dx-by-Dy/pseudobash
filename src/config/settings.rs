use {
    crate::config::args::Args,
    clap::Parser,
    serde::Deserialize,
    std::{ffi::CString, fs::File, io::BufReader, str::FromStr},
};

fn default_mode() -> Mode {
    Mode::default()
}

#[derive(Clone, Copy, Default, Deserialize)]
enum Mode {
    #[default]
    Standard,

    //#[default]
    Interactive,
}

impl TryFrom<Mode> for String {
    type Error = anyhow::Error;
    fn try_from(value: Mode) -> anyhow::Result<Self> {
        match value {
            Mode::Standard => String::from_str("s").map_err(|e| anyhow::Error::new(e)),
            Mode::Interactive => String::from_str("i").map_err(|e| anyhow::Error::new(e)),
        }
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub utils_path: CString,
    invitation_input_symbol: String,

    #[serde(default = "default_mode")]
    mode: Mode,
}

impl Settings {
    pub fn set_interactive_mode(&mut self, value: bool) {
        self.mode = match value {
            true => Mode::Interactive,
            false => Mode::Standard,
        }
    }

    pub fn is_interactive(&self) -> bool {
        match self.mode {
            Mode::Interactive => true,
            _ => false,
        }
    }

    pub fn get_invitation_input_symbol(&self) -> String {
        self.invitation_input_symbol.clone()
    }

    pub fn get_mode_input_symbol(&self) -> anyhow::Result<String> {
        self.mode.try_into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        serde_json::from_reader::<BufReader<File>, Self>(BufReader::new(
            File::open(Args::parse().config_file).expect("Failed to open config file"),
        ))
        .expect("Failed to read config file")
    }
}
