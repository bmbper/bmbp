use super::view::role_index_view;
use salvo::Router;
pub fn role_router() -> Router {
    let view_router = Router::with_path("/index.view").get(role_index_view);
    let api_router = Router::new();
    Router::new().push(view_router).push(api_router)
}
