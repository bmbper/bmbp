use axum::routing::{get, post};
use axum::Router;

use crate::index::index_api;
use crate::login::{login, login_out};
use crate::portal::{find_app, find_app_menu};

pub fn build_home_router() -> Router {
    tracing::info!("初始化应用主页路由......");
    let router = Router::new().route("/", get(index_api)).nest(
        "/portal",
        Router::new()
            .route("/login", post(login))
            .route("/logout", post(login_out))
            .route("/app", get(find_app))
            .route("/app/menu/:app_id", post(find_app_menu).get(find_app_menu)),
    );
    router
}
