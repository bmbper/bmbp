use axum::Router;

use bmbp_file::build_file_router;
use bmbp_home::build_home_router;
use bmbp_rbac::build_rbac_router;

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

    router
}
