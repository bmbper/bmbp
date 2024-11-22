use bmbp_vars::app_locale;
use rust_embed::RustEmbed;
use salvo::Router;

pub fn init_app_router() -> Router {
    let mut router = Router::new()
        .push(bmbp_ui_lib::build_router())
        .push(bmbp_ui_theme::build_router())
        .push(bmbp_framework::build_router())
        .push(bmbp_home::build_router());
    router
}

pub fn init_template() {
    bmbp_framework::build_template();
}
