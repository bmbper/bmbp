use bmbp_ui_lib::build_bmbp_ui_lib_router;
use bmbp_ui_theme::build_bmbp_ui_theme_router;
use salvo::Router;
use bmbp_home::build_bmbp_home_router;

pub fn build_app_router() -> Router {
    let mut router = Router::new()
        .push(build_bmbp_ui_lib_router())
        .push(build_bmbp_ui_theme_router())
        .push(build_bmbp_home_router());
    router
}
