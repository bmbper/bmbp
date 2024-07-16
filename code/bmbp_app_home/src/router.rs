use crate::inner::inner_script_router;
use crate::view::{home_view, login_view};
use rust_embed::RustEmbed;
use salvo::Router;

pub fn build_app_home_router() -> Router {
    let static_router = inner_script_router();
    let mut router = Router::with_path("/")
        .push(Router::with_path("/home").get(home_view))
        .push(Router::with_path("/login").get(login_view));
    router = router.push(static_router);
    router
}
