pub mod args;
pub mod environment;
pub mod settings;
pub mod utils;

use {
    crate::{
        config::{environment::Environment, settings::Settings},
        helpers::static_safe_mutable_field::StaticSafeMutableField,
    },
    lazy_static::lazy_static,
    std::ffi::CString,
};

#[derive(Default)]
pub struct Config {
    env: StaticSafeMutableField<Environment>,
    settings: StaticSafeMutableField<Settings>,
}

impl Config {
    pub fn current_env(&self) -> anyhow::Result<Vec<CString>> {
        self.env.apply_mut(|env| env.get_env())?
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}
