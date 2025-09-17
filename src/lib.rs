pub mod listener;

mod delimeter;
mod executor;
mod pb_core;
mod pipeline;
mod program;
mod static_structs;

thread_local! {
    pub static ENVIRONMENT: std::cell::RefCell<crate::static_structs::environment::Environment> = std::cell::RefCell::new(crate::static_structs::environment::Environment::default());
}

thread_local! {
    pub static DEFAULT_UTILS: std::cell::RefCell<crate::static_structs::default_utils::DefaultUtils> = std::cell::RefCell::new(crate::static_structs::default_utils::DefaultUtils::default());
}

thread_local! {
    pub static SETTINGS: std::cell::RefCell<crate::static_structs::settings::Settings> = std::cell::RefCell::new(crate::static_structs::settings::Settings::default());
}
