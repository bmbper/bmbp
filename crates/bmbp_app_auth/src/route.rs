use crate::login::{do_login, login_call_back, login_out, login_view};
use crate::middle::auth_middle;
use bmbp_abc::tera_add_template;
use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "static/bmbp_app_auth/"]
struct StaticAssets;

pub fn build_router() -> Router {
    build_template();

    let mut router = Router::new().hoop(auth_middle);
    let auth_router = Router::with_path("auth")
        .push(Router::with_path("login.view").get(login_view))
        .push(Router::with_path("login.action").post(do_login))
        .push(Router::with_path("callback.action").get(login_call_back))
        .push(
            Router::with_path("logout.action")
                .get(login_out)
                .post(login_out),
        );

    router = router.push(auth_router);
    router = router.push(
        Router::with_path("/static/bmbp_app_auth/<**path>").get(static_embed::<StaticAssets>()),
    );
    router
}

#[derive(RustEmbed)]
#[folder = "web/templates/"]
#[prefix = "bmbp_app_auth/"]
pub(crate) struct PageAssets;

pub fn build_template() {
    for file_path in PageAssets::iter() {
        tera_add_template(file_path.as_ref(), PageAssets::get(file_path.as_ref()));
    }
}
