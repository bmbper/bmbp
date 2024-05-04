use crate::app::web::*;
use salvo::Router;

pub fn app_router() -> Router {
    Router::new()
        .push(Router::with_path("/page").post(find_app_page))
        .push(Router::with_path("/list").post(find_app_list))
        .push(Router::with_path("/save").post(save_app))
        .push(Router::with_path("/info/id/<dataId>").post(find_app_info))
        .push(Router::with_path("/enable/<dataId>").post(enable_app))
        .push(Router::with_path("/disable/<dataId>").post(disable_app))
        .push(Router::with_path("/remove/<dataId>").post(remove_app))
}
