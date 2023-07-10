use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

#[derive(RustEmbed)]
#[folder = "../../webapp/app/"]
struct Asset;

pub fn build_embed_router() -> Router {
    Router::with_path("/bmbp/ui/<**path>").get(static_embed::<Asset>().fallback("index.html"))
}
