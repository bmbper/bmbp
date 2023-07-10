use crate::user::web::*;
use salvo::Router;

pub fn user_router() -> Router {
    Router::with_path("/find/user/tree").get(find_user_tree)
}
