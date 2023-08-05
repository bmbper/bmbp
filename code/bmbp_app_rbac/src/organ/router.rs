use crate::organ::view::*;
use crate::organ::web::*;

use salvo::Router;
pub fn organ_router() -> Router {
    let organ_view_router =
        Router::new().push(Router::with_path("index.view").get(organ_index_view));

    let organ_api_router = Router::new()
        .push(
            Router::with_path("/find/tree")
                .post(find_organ_tree)
                .get(find_organ_tree),
        )
        .push(Router::with_path("/find/tree/id/<id>").get(find_organ_tree_start_with_id))
        .push(Router::with_path("/find/tree/code/<code>").post(find_organ_tree_start_with_code))
        .push(Router::with_path("/find/page").post(find_organ_page))
        .push(Router::with_path("/find/page/parent/<parent>").post(find_organ_page_by_parent))
        .push(
            Router::with_path("/find/list")
                .post(find_organ_list)
                .get(find_organ_list),
        )
        .push(
            Router::with_path("/find/list/parent/<parent>")
                .post(find_organ_list_by_parent)
                .get(find_organ_list_by_parent),
        )
        .push(
            Router::with_path("/find/info/id/<id>")
                .post(find_organ_info_by_id)
                .get(find_organ_info_by_id),
        )
        .push(
            Router::with_path("/find/info/code/<organ_code>")
                .post(find_organ_info_by_code)
                .get(find_organ_info_by_code),
        )
        .push(Router::with_path("/save").post(save_organ))
        .push(Router::with_path("/insert").post(insert_organ))
        .push(Router::with_path("/update").post(update_organ))
        .push(Router::with_path("/update/id/<id>").post(update_organ_by_id))
        .push(Router::with_path("/update/parent/<id>/<parent>").post(update_organ_parent))
        .push(Router::with_path("/enable/id/<id>").post(enable_organ_by_id))
        .push(Router::with_path("/disable/id/<id>").post(disable_organ_by_id))
        .push(Router::with_path("/remove").post(remove_organ))
        .push(Router::with_path("/remove/id/<id>").post(remove_organ_by_id))
        .push(Router::with_path("/remove/batch/id/<id>").post(batch_remove_organ_by_id));

    Router::new().push(organ_view_router).push(organ_api_router)
}
