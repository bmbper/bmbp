use salvo::Router;

use super::view::menu_index_view;
pub fn res_menu_router() -> Router {
    let view_router = Router::with_path("/index.view").get(menu_index_view);
    let api_router = Router::new();
    Router::with_path("/menu")
        .push(view_router)
        .push(api_router)
}
