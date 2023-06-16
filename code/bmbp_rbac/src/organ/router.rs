use crate::organ::web::*;
use axum::{
    routing::{get, post},
    Router,
};
pub fn organ_router() -> Router {
    Router::new()
        .route("/find/tree", post(find_organ_tree))
        .route(
            "/find/tree/id/:id",
            post(find_organ_tree_start_with_id).get(find_organ_tree_start_with_id),
        )
        .route(
            "/find/tree/code/:code",
            post(find_organ_tree_start_with_code),
        )
        .route(
            "/find/tree/parent/:parent",
            post(find_organ_tree_start_with_parent),
        )
        .route("/find/page", post(find_organ_page))
        .route("/find/page/parent/:parent", post(find_organ_page_by_parent))
        .route("/find/list", post(find_organ_list))
        .route("/find/list/parent/:parent", post(find_organ_list_by_parent))
        .route("/find/info/id/:id", get(find_organ_info_by_id))
        .route("/find/info/code/:organ_code", get(find_organ_info_by_code))
        .route("/save", post(save_organ))
        .route("/insert", post(insert_organ))
        .route("/update", post(update_organ))
        .route("/update/id/:id", post(update_organ_by_id))
        .route("/update/parent/:id/:parent", post(update_organ_parent))
        .route("/enable/id/:id", post(enable_organ_by_id))
        .route("/disable/id/:id", post(disable_organ_by_id))
        .route("/remove/id/:id", post(remove_organ_by_id))
        .route("/remove/batch/id/:id", post(batch_remove_organ_by_id))
}