use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "static/theme"]
struct StaticAssets;

pub fn build_router() -> Router {
    Router::with_path("/static/theme/<**path>").get(static_embed::<StaticAssets>())
}
