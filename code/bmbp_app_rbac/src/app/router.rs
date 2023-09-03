use crate::app::view::*;
use salvo::Router;

use super::web::*;

pub fn app_router() -> Router {
    let view_router = Router::new()
        .push(Router::with_path("/index.view").get(app_index_view))
        .push(Router::with_path("/config.view").get(app_config_view))
        .push(Router::with_path("/base.view").get(app_base_view));

    let api_router = Router::new()
        .push(Router::with_path("/find/page").post(find_app_page))
        .push(Router::with_path("/find/list").post(find_app_list))
        .push(
            Router::with_path("/find/info/<recordId>")
                .get(find_app_info)
                .post(find_app_info),
        )
        .push(Router::with_path("/save").post(save_app))
        .push(Router::with_path("/enable/<recordId>").post(enable_app))
        .push(Router::with_path("/disable/<recordId>").post(disable_app))
        .push(Router::with_path("/restart/<recordId>").post(restart_app))
        .push(Router::with_path("/delete/<recordId>").post(delete_app))
        .push(Router::with_path("/batch/enable").post(batch_enable_app))
        .push(Router::with_path("/batch/disable").post(batch_disable_app))
        .push(Router::with_path("/batch/delete").post(batch_delete_app));
    Router::new().push(view_router).push(api_router)
}
