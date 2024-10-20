use salvo::Router;

use crate::organ::web::*;

pub fn organ_router() -> Router {
    Router::new()
        .push(Router::with_path("tree").post(find_organ_tree))
        .push(Router::with_path("tree/id/<dataId>").post(find_organ_tree_by_id))
        .push(Router::with_path("tree/crates/<crates>").post(find_organ_tree_by_code))
        .push(Router::with_path("tree/exclude/person").post(find_organ_tree_exclude_person))
        .push(Router::with_path("page").post(find_organ_page))
        .push(Router::with_path("list").post(find_organ_list))
        .push(Router::with_path("save").post(save_organ))
        .push(Router::with_path("insert").post(insert_organ))
        .push(Router::with_path("update").post(update_organ))
        .push(Router::with_path("info/id/<dataId>").post(find_organ_info))
        .push(Router::with_path("enable/<dataId>").post(enable_organ))
        .push(Router::with_path("disable/<dataId>").post(disable_organ))
        .push(Router::with_path("remove/<dataId>").post(remove_organ))
        .push(Router::with_path("change/parent/tree/<dataId>").post(find_organ_tree_exclude_by_id))
        .push(Router::with_path("change/parent/save/<dataId>").post(save_organ_parent))
}
