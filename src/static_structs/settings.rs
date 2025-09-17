use std::{cell::Cell, str::FromStr};

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

#[derive(Clone)]
pub struct Settings {
    invitation_input_symbol: String,
    mode: Cell<Mode>,
}

impl Settings {
    pub fn set_interactive_mode(&mut self, value: bool) {
        self.mode.replace(match value {
            true => Mode::Interactive,
            false => Mode::Standard,
        });
    }

    pub fn is_interactive(&self) -> bool {
        match self.mode.get() {
            Mode::Interactive => true,
            _ => false,
        }
    }

    pub fn get_invitation_input(&self) -> String {
        format!(
            "{}{}{}",
            String::from(self.mode.get()),
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
