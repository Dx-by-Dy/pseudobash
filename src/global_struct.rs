pub mod default_utils;
pub mod environment;
pub mod settings;

use crate::global_struct::{
    default_utils::DefaultUtils, environment::Environment, settings::Settings,
};

#[derive(Default)]
pub struct GS {
    pub(crate) environment: Environment,
    pub(crate) settings: Settings,
    pub(crate) default_utils: DefaultUtils,
}
