use super::web::*;
use salvo::Router;

pub fn role_user_router() -> Router {
    Router::new()
        .push(Router::with_path("/find/role/page").post(find_role_page))
        .push(Router::with_path("/find/role/tree/checked").post(find_role_tree_checked))
        .push(Router::with_path("/remove/role/<recordId>").post(remove_role_by_id))
        .push(Router::with_path("/add/user/roles").post(add_user_roles))
}
