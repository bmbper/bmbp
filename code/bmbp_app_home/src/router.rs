use crate::home::home_view;
use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;
#[derive(RustEmbed)]
#[folder = "script"]
struct HomeScriptAsset;
pub fn build_app_home_router() -> Router {
    let static_router =
        Router::with_path("/ui/lib/home/<**path>").get(static_embed::<HomeScriptAsset>());
    let mut router = Router::with_path("/").push(Router::with_path("/home").get(home_view));
    router = router.push(static_router);
    router
}
