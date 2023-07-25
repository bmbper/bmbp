mod types;
use once_cell::sync::Lazy;
pub use types::*;
mod vars;
pub use vars::*;
mod utils;
use rust_embed::RustEmbed;
use tera::Tera;
pub use utils::*;

/// 全局的模板
pub static BMBP_TEMPLATE: Lazy<Tera> = Lazy::new(|| {
    let mut tp = Tera::new("webapp/template/**/*").unwrap();
    tp.autoescape_on(vec![".html"]);
    tp
});
