use super::view::*;
use super::web::*;

use salvo::Router;
pub fn menu_router() -> Router {
    let menu_view_router = Router::new().push(Router::with_path("index.view").get(menu_index_view));

    let menu_api_router = Router::new()
        .push(
            Router::with_path("/find/tree")
                .post(find_menu_tree)
                .get(find_menu_tree),
        )
        .push(Router::with_path("/find/tree/id/<id>").get(find_menu_tree_start_with_id))
        .push(
            Router::with_path("/find/tree/with/out/app/<appId>/id/<id>")
                .get(find_menu_tree_with_out_id),
        )
        .push(Router::with_path("/find/tree/code/<code>").post(find_menu_tree_start_with_code))
        .push(Router::with_path("/find/page").post(find_menu_page))
        .push(Router::with_path("/find/page/parent/<parent>").post(find_menu_page_by_parent))
        .push(
            Router::with_path("/find/list")
                .post(find_menu_list)
                .get(find_menu_list),
        )
        .push(
            Router::with_path("/find/list/parent/<parent>")
                .post(find_menu_list_by_parent)
                .get(find_menu_list_by_parent),
        )
        .push(
            Router::with_path("/find/info/id/<recordId>")
                .post(find_menu_info_by_id)
                .get(find_menu_info_by_id),
        )
        .push(
            Router::with_path("/find/info/code/<menuCode>")
                .post(find_menu_info_by_code)
                .get(find_menu_info_by_code),
        )
        .push(Router::with_path("/save").post(save_menu))
        .push(Router::with_path("/insert").post(insert_menu))
        .push(Router::with_path("/update").post(update_menu))
        .push(Router::with_path("/update/id/<recordId>").post(update_menu_by_id))
        .push(Router::with_path("/update/parent/<recordId>/<parent>").post(update_menu_parent))
        .push(Router::with_path("/enable/id/<recordId>").post(enable_menu_by_id))
        .push(Router::with_path("/disable/id/<recordId>").post(disable_menu_by_id))
        .push(Router::with_path("/remove").post(remove_menu))
        .push(Router::with_path("/remove/id/<recordId>").post(remove_menu_by_id))
        .push(Router::with_path("/remove/batch/id/<recordId>").post(batch_remove_menu_by_id));

    Router::new().push(menu_view_router).push(menu_api_router)
}
