mod action;
mod init;
mod view;
use bmbp_ui_lib::build_ui_lib_router;
use init::build_init_script_router;
use salvo::Router;
use view::init_view;
pub fn build_init_router() -> Router {
    let mut router = Router::new();
    router = router.push(build_ui_lib_router());
    router = router.push(build_init_script_router());
    router = router.get(init_view);
    router = router.push(Router::with_path("/save/config.do").post(action::save_config));
    router
}
