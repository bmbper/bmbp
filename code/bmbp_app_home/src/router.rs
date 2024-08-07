use crate::inner::inner_script_router;
use crate::view::{home_view, login_view};
use salvo::Router;

pub fn build_app_home_router() -> Router {
    let static_router = inner_script_router();
    let mut router = Router::new()
        .push(Router::with_path("home.view").get(home_view))
        .push(Router::with_path("login.view").get(login_view));
    router = router.push(static_router);
    router
}
