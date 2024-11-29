use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "static/bmbp_ui_theme"]
struct StaticAssets;

pub fn build_router() -> Router {
    Router::with_path("/static/bmbp_ui_theme/<**path>").get(static_embed::<StaticAssets>())
}
