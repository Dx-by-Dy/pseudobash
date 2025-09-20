pub mod global_struct;
pub mod listener;

mod executor;
mod parser;
mod pb_core;
mod pipeline;
mod program;
//mod static_structs;

// pub static DEFAULT_UTILS: std::sync::LazyLock<
//     std::sync::Mutex<crate::static_structs::default_utils::DefaultUtils>,
// > = std::sync::LazyLock::new(|| {
//     std::sync::Mutex::new(crate::static_structs::default_utils::DefaultUtils::default())
// });

// pub static SETTINGS: std::sync::LazyLock<
//     std::sync::Mutex<crate::static_structs::settings::Settings>,
// > = std::sync::LazyLock::new(|| {
//     std::sync::Mutex::new(crate::static_structs::settings::Settings::default())
// });
