use salvo::Router;

use crate::role::web::{
    disable_role, enable_role, find_role_info, find_role_list, find_role_page, remove_role,
    save_role,
};

pub fn role_router() -> Router {
    Router::new()
        .push(Router::with_path("/page").post(find_role_page))
        .push(Router::with_path("/list").post(find_role_list))
        .push(Router::with_path("/save").post(save_role))
        .push(Router::with_path("/info/id/<dataId>").post(find_role_info))
        .push(Router::with_path("/enable/<dataId>").post(enable_role))
        .push(Router::with_path("/disable/<dataId>").post(disable_role))
        .push(Router::with_path("/remove/<dataId>").post(remove_role))
}
