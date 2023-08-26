use crate::role::view::*;
use crate::role::web::*;

use salvo::Router;
pub fn role_router() -> Router {
    let role_view_router = Router::new().push(Router::with_path("index.view").get(role_index_view));

    let role_api_router = Router::new()
        .push(
            Router::with_path("/find/tree")
                .post(find_role_tree)
                .get(find_role_tree),
        )
        .push(Router::with_path("/find/tree/id/<id>").get(find_role_tree_start_with_id))
        .push(Router::with_path("/find/tree/with/out/id/<id>").get(find_role_tree_with_out_id))
        .push(Router::with_path("/find/tree/code/<code>").post(find_role_tree_start_with_code))
        .push(Router::with_path("/find/page").post(find_role_page))
        .push(Router::with_path("/find/page/parent/<parent>").post(find_role_page_by_parent))
        .push(
            Router::with_path("/find/list")
                .post(find_role_list)
                .get(find_role_list),
        )
        .push(
            Router::with_path("/find/list/parent/<parent>")
                .post(find_role_list_by_parent)
                .get(find_role_list_by_parent),
        )
        .push(
            Router::with_path("/find/info/id/<recordId>")
                .post(find_role_info_by_id)
                .get(find_role_info_by_id),
        )
        .push(
            Router::with_path("/find/info/code/<roleCode>")
                .post(find_role_info_by_code)
                .get(find_role_info_by_code),
        )
        .push(Router::with_path("/save").post(save_role))
        .push(Router::with_path("/insert").post(insert_role))
        .push(Router::with_path("/update").post(update_role))
        .push(Router::with_path("/update/id/<recordId>").post(update_role_by_id))
        .push(Router::with_path("/update/parent/<recordId>/<parent>").post(update_role_parent))
        .push(Router::with_path("/enable/id/<recordId>").post(enable_role_by_id))
        .push(Router::with_path("/disable/id/<recordId>").post(disable_role_by_id))
        .push(Router::with_path("/remove").post(remove_role))
        .push(Router::with_path("/remove/id/<recordId>").post(remove_role_by_id))
        .push(Router::with_path("/remove/batch/id/<recordId>").post(batch_remove_role_by_id));

    Router::new().push(role_view_router).push(role_api_router)
}
