use bmbp_app_common::build_common_router;
use bmbp_app_file::build_file_router;
use bmbp_app_portal::build_home_router;
use bmbp_app_rbac::build_rbac_router;
use salvo::{
    cors::{Cors, CorsHandler},
    handler,
    hyper::Method,
    logging::Logger,
    Router,
};
/// init_webapp_router web应用的路由注册
pub fn init_webapp_router() -> Router {
    let mut router = Router::new();
    // 增加日志
    router = router.hoop(Logger::new());

    // 构建全局静态文件路由
    let common_router = build_common_router();
    router = router.push(common_router);

    let static_file_router = build_file_router();
    router = router.push(static_file_router);

    // 引入主模块路由，用于处理登录，首页相关逻辑
    let api_home_router = build_home_router();
    router = router.push(api_home_router);

    // 引入权限管理模块路由
    let api_rbac_router = build_rbac_router();
    router = router.push(api_rbac_router);
    let cors_handler = build_router_cors();
    router = router.hoop(cors_handler).options(handler::empty());
    router
}

fn build_router_cors() -> CorsHandler {
    Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .into_handler()
}
