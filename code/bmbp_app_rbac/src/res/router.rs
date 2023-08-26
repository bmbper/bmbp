use salvo::Router;

use super::menu::menu_router;
pub fn res_router() -> Router {
    Router::with_path("/menu").push(menu_router())
}
