use salvo::catcher::Catcher;
use salvo::cors::{Cors, CorsHandler};
use salvo::http::{Method, StatusCode};
use salvo::logging::Logger;
use salvo::prelude::Json;
use salvo::{handler, FlowCtrl, Response, Router, Service};

use bmbp_app_base::build_app_base_router;
use bmbp_app_common::{RespCode, RespVo};
use bmbp_app_file::build_file_router;
use bmbp_app_home::build_app_home_router;
use bmbp_app_portal::build_home_router;
use bmbp_app_rbac::build_rbac_router;
use bmbp_app_setting::build_setting_router;
use bmbp_ui_lib::build_ui_lib_router;

/// init_webapp_router web应用的路由注册
pub fn init_webapp_router() -> Service {
    let mut router = Router::new();
    // 增加日志
    router = router.hoop(Logger::new());

    // 首页
    router = router.push(build_app_home_router());

    // web 公共包
    router = router.push(build_ui_lib_router());

    let file_router = build_file_router();
    router = router.push(file_router);

    //公共模块路由
    let app_base_router = build_app_base_router();
    router = router.push(app_base_router);

    // 系统配置路由，用于处理字典、系统参数、行政区划等
    let api_setting_router = build_setting_router();
    router = router.push(api_setting_router);

    // 引入主模块路由，用于处理登录，首页相关逻辑
    let api_home_router = build_home_router();
    router = router.push(api_home_router);

    // 引入权限管理模块路由
    let api_rbac_router = build_rbac_router();
    router = router.push(api_rbac_router);

    // 引用低代码路由
    let runtime_router = bmbp_dev::builder_dev_router();
    router = router.push(runtime_router);

    let cors_handler = build_router_cors();
    router = router.hoop(cors_handler).options(handler::empty());
    Service::new(router).catcher(Catcher::default().hoop(err_handler))
}

fn build_router_cors() -> CorsHandler {
    Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .into_handler()
}

#[handler]
async fn err_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    let mut resp: RespVo<std::string::String> = RespVo::fail();
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        resp.set_code(RespCode::NotFound);
        resp.set_msg("404: 接口不存在");
    }
    res.render(Json(resp));
    ctrl.skip_rest();
}
