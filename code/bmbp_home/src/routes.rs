use axum::{routing::get, Router};

use crate::index::index_api;

pub fn build_home_router() -> Router {
    tracing::info!("初始化应用主页路由......");
    let mut router = Router::new();
    router = router.route("/echo", get(index_api));
    router
}
