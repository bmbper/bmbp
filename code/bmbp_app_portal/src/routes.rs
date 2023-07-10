use crate::index::index_api;
use crate::login::{login, login_out};
use crate::portal::{find_app, find_app_menu};
use salvo::Router;

pub fn build_home_router() -> Router {
    Router::new()
        .push(Router::new().get(index_api))
        .push(Router::with_path("/index").get(index_api))
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
