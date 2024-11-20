use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "web/static/theme"]
struct StaticAssets;

pub fn build_bmbp_ui_theme_router() -> Router {
    Router::with_path("/static/theme/<**path>").get(static_embed::<StaticAssets>())
}
