mod app;
mod cache;
mod data;
mod env;
mod init;
mod plugin;

pub use app::BmbpApp;
rust_i18n::i18n!("locales", fallback = "zh_cn");
