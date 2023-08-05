use crate::user::web::*;
use salvo::Router;

use super::view::user_index_view;

pub fn user_router() -> Router {
    let view_router = Router::with_path("/index.view").get(user_index_view);
    let api_router = Router::with_path("/find/user/tree").get(find_user_tree);
    Router::new().push(view_router).push(api_router)
}
