use crate::app::model::{BmbpRbacApp, BmbpRbacAppMenu};
use salvo::Router;
pub fn app_router() -> Router {
    let router = BmbpRbacApp::build_router()
        .push(Router::with_path("menu").push(BmbpRbacAppMenu::build_router()));
    router
}
