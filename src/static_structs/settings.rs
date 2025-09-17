use std::str::FromStr;

#[derive(Clone, Copy, Default, Debug)]
enum Mode {
    #[default]
    Standard,

    Interactive,
}

impl From<Mode> for String {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Standard => String::from_str("s").unwrap(),
            Mode::Interactive => String::from_str("i").unwrap(),
        }
    }
}

pub struct Settings {
    invitation_input_symbol: String,
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
        println!("{:?}", self.mode);
        match self.mode {
            Mode::Interactive => true,
            _ => false,
        }
    }

    pub fn get_invitation_input(&self) -> String {
        format!(
            "{}{}{}",
            String::from(self.mode),
            self.invitation_input_symbol,
            self.invitation_input_symbol
        )
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            invitation_input_symbol: ">".to_string(),
            mode: Default::default(),
        }
    }
}
