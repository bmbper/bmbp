use crate::login::login_view;
use salvo::Router;
pub fn build_app_base_router() -> Router {
    Router::new().push(Router::with_path("/login").get(login_view))
}
