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

pub fn build_common_router() -> Router {
    let mut router = Router::new();
    if is_production() {
        router =
            router.push(Router::with_path("/static/<**path>").get(static_embed::<EmbedStatic>()));
    } else {
        router = router
            .push(Router::with_path("/static/<**path>").get(StaticDir::new(vec!["webapp/static"])));
    }
    router
}

fn is_production() -> bool {
    let env = global_hash_map_vars("bmbp_app".to_string(), "app_env".to_string());
    tracing::info!("app_env:{},is_prod:{}", env,env.eq_ignore_ascii_case("prod"));
    env.eq_ignore_ascii_case("prod")
}
