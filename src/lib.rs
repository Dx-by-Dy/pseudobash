pub mod listener;

mod delimeter;
mod executor;
mod pb_core;
mod pipeline;
mod program;
mod static_structs;

pub static ENVIRONMENT: std::sync::LazyLock<
    std::sync::Mutex<crate::static_structs::environment::Environment>,
> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(crate::static_structs::environment::Environment::default())
});

pub static DEFAULT_UTILS: std::sync::LazyLock<
    std::sync::Mutex<crate::static_structs::default_utils::DefaultUtils>,
> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(crate::static_structs::default_utils::DefaultUtils::default())
});

pub static SETTINGS: std::sync::LazyLock<
    std::sync::Mutex<crate::static_structs::settings::Settings>,
> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(crate::static_structs::settings::Settings::default())
});
