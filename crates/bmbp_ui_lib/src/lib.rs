use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "lib"]
struct Asset;
pub fn build_ui_lib_router() -> Router {
    Router::with_path("ui/lib/<**path>").get(static_embed::<Asset>())
}
