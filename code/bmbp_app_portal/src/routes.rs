use crate::index_view::favicon;
use crate::index_view::index_view;
use crate::login::{login, login_view};
use crate::portal::portal_view;
use salvo::Router;

pub fn build_home_router() -> Router {
    let view_router = Router::new()
        .push(Router::new().get(index_view))
        .push(Router::with_path("/favicon.ico").get(favicon))
        .push(Router::with_path("/login.view").get(login_view))
        .push(Router::with_path("/portal.view").get(portal_view))
        .push(Router::with_path("/portal/index.view").get(index_view));

    let api_router = Router::new()
        .push(Router::with_path("/api/v1").push(Router::with_path("/login.do").post(login)));

    Router::new().push(view_router).push(api_router)
}
