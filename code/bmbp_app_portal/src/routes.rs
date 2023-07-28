use crate::index_view::favicon;
use crate::index_view::index_view;
use crate::login::{login, login_out, login_view};
use crate::portal::{find_app, find_app_menu, portal_view};
use salvo::Router;

pub fn build_home_router() -> Router {
    Router::new()
        .push(Router::new().get(index_view))
        .push(Router::with_path("/favicon.ico").get(favicon))
        .push(Router::with_path("/index.view").get(index_view))
        .push(Router::with_path("/login.view").get(login_view))
        .push(Router::with_path("/portal.view").get(portal_view))
        .push(
            Router::with_path("/portal")
                .push(Router::with_path("/login").post(login))
                .push(Router::with_path("/logout").post(login_out).get(login_out))
                .push(Router::with_path("/app").post(find_app).get(find_app))
                .push(
                    Router::with_path("/app/menu/<app_id>")
                        .post(find_app_menu)
                        .get(find_app_menu),
                ),
        )
}
