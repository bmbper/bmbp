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
pub static BMBP_TEMPLATE: Lazy<Tera> = Lazy::new(|| build_development_template());

#[allow(dead_code)]
fn build_production_template() -> Tera {
    let mut tp = Tera::default();
    for file in EmbedTemplate::iter() {
        let file_path = file.as_ref();
        let file_data = EmbedTemplate::get(file_path).unwrap();
        let file_str = std::str::from_utf8(file_data.data.as_ref()).unwrap();
        tp.add_raw_template(file.as_ref(), file_str).unwrap();
    }
    tp.autoescape_on(vec![".html"]);
    tp
}
#[allow(dead_code)]
fn build_development_template() -> Tera {
    let mut tp = Tera::new("webapp/template/**/*").unwrap();
    tp.autoescape_on(vec![".html"]);
    tp.full_reload().unwrap();
    tp
}

#[derive(RustEmbed)]
#[folder = "../../webapp/template/"]
pub struct EmbedTemplate;

#[derive(RustEmbed)]
#[folder = "../../webapp/static/"]
pub struct EmbedStatic;
