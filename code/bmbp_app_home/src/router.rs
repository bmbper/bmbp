use crate::home::home_view;
use salvo::Router;

pub fn build_app_home_router() -> Router {
    Router::with_path("/").push(Router::with_path("/home").get(home_view))
}
