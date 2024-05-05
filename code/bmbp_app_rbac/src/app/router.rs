use crate::app::model::BmbpRbacApp;
use salvo::Router;
///
///
///
/// /find/all/page
/// /find/remove/page
/// /find/all/page
pub fn app_router() -> Router {
    let mut router = BmbpRbacApp::build_router();
    router
}
