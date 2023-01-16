use axum::body::BoxBody;
use axum::http::{Request, Uri};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{body, middleware, Router};
use tower::Layer;
use tower_http::cors::{AllowCredentials, AllowOrigin, Any, CorsLayer};

use bmbp_auth::BmbpAuthLayer;
use bmbp_file::build_file_router;
use bmbp_home::build_home_router;
use bmbp_rbac::build_rbac_router;
use bmbp_types::{BmbpResp, RespVo};

/// init_webapp_router web应用的路由注册
pub fn init_webapp_router(mut router: Router) -> Router {
    // 构建全局静态文件路由
    let static_file_router = build_file_router();
    router = router.merge(static_file_router);

    // 引入主模块路由，用于处理登录，首页相关逻辑
    let api_home_router = build_home_router();
    router = router.merge(api_home_router);

    // 引入权限管理模块路由
    let api_rbac_router = build_rbac_router();
    router = router.merge(api_rbac_router);

    // 异常处理
    router = router.fallback(not_found);

    //认证处理
    router = router.layer(BmbpAuthLayer::new());
    // 跨域处理
    router = router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
    router
}

async fn not_found(url: Uri) -> BmbpResp<impl IntoResponse> {
    Ok(RespVo::<String>::fail_msg_data(
        format!("404:请求地址不存在"),
        url.to_string(),
    ))
}
