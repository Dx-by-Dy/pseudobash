pub mod args;
pub mod environment;
pub mod settings;
pub mod utils;

use {
    crate::{
        config::{environment::Environment, settings::Settings, utils::Utils},
        helpers::static_safe_mutable_field::StaticSafeMutableField,
    },
    lazy_static::lazy_static,
    std::ffi::CString,
};

pub struct Config {
    env: StaticSafeMutableField<Environment>,
    settings: StaticSafeMutableField<Settings>,
    utils: StaticSafeMutableField<Utils>,
}

impl Config {
    pub fn current_env(&self) -> anyhow::Result<Vec<CString>> {
        self.env.apply_mut(|e| e.get_env())?
    }

    pub fn current_dir(&self) -> anyhow::Result<CString> {
        self.env.apply(|e| e.current_dir().clone())
    }

    pub fn is_interactive(&self) -> anyhow::Result<bool> {
        self.settings.apply(|s| s.interactive)
    }

    pub fn get_full_path<'a>(&self, name: &'a mut Vec<u8>) -> anyhow::Result<&'a mut Vec<u8>> {
        self.utils.apply(|u| u.get_full_path(name))?
    }

    pub fn set_interactive(&self, value: bool) -> anyhow::Result<()> {
        self.settings.apply_mut(|s| s.interactive = value)
    }
}

impl Default for Config {
    fn default() -> Self {
        let settings = Settings::default();
        Self {
            env: StaticSafeMutableField::new(Environment::new().unwrap()),
            utils: StaticSafeMutableField::new(Utils::new(&settings.utils_path).unwrap()),
            settings: StaticSafeMutableField::new(settings),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}
