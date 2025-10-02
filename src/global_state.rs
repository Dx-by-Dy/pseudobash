mod environment;
mod settings;

use crate::global_state::{environment::Environment, settings::Settings};

#[derive(Default)]
pub struct GlobalState {
    pub(crate) environment: Environment,
    pub(crate) settings: Settings,
}
