use crate::home::{find_nav_app, find_nav_app_menu};
use axum::routing::any;
use axum::{routing::get, Router};

use crate::index::index_api;

pub fn build_home_router() -> Router {
    tracing::info!("初始化应用主页路由......");

    // 导航接口
    let nav_router = Router::new()
        .route("/app", any(find_nav_app))
        .route("/app/menu", any(find_nav_app_menu));
    // 消息接口
    let msg_router = Router::new();
    // 个人信息
    let person_router = Router::new();
    // 登录
    let login_router = Router::new();

    let mut router = Router::new();
    router = router
        .route("/echo", get(index_api))
        .nest("/home/nav", nav_router)
        .nest("/home/msg", msg_router)
        .nest("/home/person", person_router)
        .nest("/home/login", login_router);
    router
}
