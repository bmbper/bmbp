use crate::user::web::*;
use salvo::Router;

use super::view::user_index_view;

pub fn user_router() -> Router {
    let view_router = Router::with_path("/index.view").get(user_index_view);
    let api_router = Router::new()
        .push(Router::with_path("/find/user/page").get(find_user_page))
        .push(Router::with_path("/find/user/info/<recrodId>").get(find_user_info_by_id))
        .push(Router::with_path("/save").post(save_user))
        .push(Router::with_path("/update/reset/password/<recrodId>").get(reset_user_password))
        .push(Router::with_path("/update/organ/<recordId>/<organId>").post(update_user_organ))
        .push(Router::with_path("/enable/id/<recordId>").post(enable_user_by_id))
        .push(Router::with_path("/disable/id/<recordId>").post(disable_user_by_id))
        .push(Router::with_path("/remove").post(remove_user))
        .push(Router::with_path("/remove/id/<recordId>").post(remove_user_by_id))
        .push(Router::with_path("/remove/batch/id/<recordId>").post(batch_remove_user_by_id));

    Router::new().push(view_router).push(api_router)
}
