extern crate core;

mod types;
use once_cell::sync::Lazy;
pub use types::*;
mod vars;
pub use vars::*;
mod utils;
use rust_embed::RustEmbed;
use salvo::{
    serve_static::{static_embed, StaticDir},
    Router,
};
use tera::Tera;
pub use utils::*;
use vars::map::global_hash_map_vars;

/// 全局的模板
pub static BMBP_TEMPLATE: Lazy<Tera> = Lazy::new(|| {
    if is_production() {
        build_production_template()
    } else {
        build_development_template()
    }
});
/// 生产模式下 二进制包的模板
fn build_production_template() -> Tera {
    let mut tp = Tera::default();
    for file in EmbedTemplate::iter() {
        let file_path = file.as_ref();
        let file_data = EmbedTemplate::get(file_path).unwrap();
        let file_str = std::str::from_utf8(file_data.data.as_ref()).unwrap();
        tp.add_raw_template(file.as_ref(), file_str).unwrap();
    }
    tp.autoescape_on(vec![]);
    tp
}
/// 开发模式下的模板
fn build_development_template() -> Tera {
    let mut tp = Tera::new("bmbp_web/pages/**/*").unwrap();
    tp.autoescape_on(vec![]);
    tp.full_reload().unwrap();
    tp
}
/// 模板文件嵌入
#[derive(RustEmbed)]
#[folder = "../../bmbp_web/pages/"]
pub struct EmbedTemplate;

/// 静态文件嵌入
#[derive(RustEmbed)]
#[folder = "../../bmbp_web/assets/"]
pub struct EmbedStatic;

pub fn build_common_router() -> Router {
    let mut router = Router::new();
    if is_production() {
        router =
            router.push(Router::with_path("/assets/<**path>").get(static_embed::<EmbedStatic>()));
    } else {
        router = router.push(
            Router::with_path("/assets/<**path>").get(StaticDir::new(vec!["bmbp_web/assets"])),
        );
    }
    router
}

fn is_production() -> bool {
    let env = global_hash_map_vars("bmbp_app".to_string(), "app_env".to_string());
    tracing::info!(
        "app_env:{},is_prod:{}",
        env,
        env.eq_ignore_ascii_case("prod")
    );
    env.eq_ignore_ascii_case("prod")
}
