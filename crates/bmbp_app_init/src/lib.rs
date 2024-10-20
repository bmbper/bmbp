mod action;
mod init;
mod view;
use bmbp_ui_lib::build_ui_lib_router;
use init::build_init_script_router;
use salvo::Router;
use view::{index_view, root_view};

pub fn build_init_config_router() -> Router {
    build_init_router()
}
pub fn build_init_app_router() -> Router {
    let mut router = build_init_router();
    router = router.get(root_view);
    router
}

pub fn build_init_router() -> Router {
    let mut router = Router::new();
    router = router.push(build_ui_lib_router());
    router = router.push(build_init_script_router());
    router = router.push(Router::with_path("init/index.view").get(index_view));
    router = router.push(Router::with_path("init/save/config.do").post(action::save_config));
    router =
        router.push(Router::with_path("init/valid/datasource.do").post(action::valid_datasource));
    router
}
