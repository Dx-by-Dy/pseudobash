#[derive(Clone, Copy, Default, Debug)]
pub struct Mode {
    pub(crate) interactive: bool,
    pub(crate) xargs: bool,
}

impl From<Mode> for String {
    fn from(value: Mode) -> Self {
        let mut output = String::new();
        if value.interactive {
            output.push('i');
        }
        if value.xargs {
            output.push('x');
        }
        output
    }
}

#[derive(Clone)]
pub struct Settings {
    invitation_input_symbol: String,
    pub(crate) mode: Mode,
}

impl Settings {
    pub fn set_interactive_mode(&mut self, value: bool) {
        self.mode.interactive = value
    }

    pub fn set_xargs_mode(&mut self, value: bool) {
        self.mode.xargs = value
    }

    pub fn get_invitation_input(&self) -> String {
        let str_mode = String::from(self.mode);
        let mut output = String::new();
        if str_mode.len() > 0 {
            output.push_str(format!("\x1b[0;1;31m({}) ", str_mode).as_str());
        }
        output.push_str(
            format!(
                "\x1b[0;1;37m{}{}{}\x1b[0m",
                self.invitation_input_symbol,
                self.invitation_input_symbol,
                self.invitation_input_symbol
            )
            .as_str(),
        );
        output
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
