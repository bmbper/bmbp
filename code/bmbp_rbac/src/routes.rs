use axum::{
    routing::{get, post},
    Router,
};

use crate::organ_route::*;

pub fn build_rbac_router() -> Router {
    tracing::info!("初始化权限管理模块路由......");
    Router::new().nest(
        "/rbac",
        Router::new()
            .nest(
                "/v1",
                Router::new()
                    .nest(
                        "/organ",
                        Router::new()
                            .route("/find/tree", post(find_organ_tree))
                            .route("/find/tree/id/:id", post(find_organ_tree_start_with_id))
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
                            .route("/remove/batch/id/:id", post(batch_remove_organ_by_id)),
                    )
                    .nest("/user", Router::new())
                    .nest("/role", Router::new())
                    .nest("/res", Router::new())
                    .nest("/api", Router::new())
                    .nest("/data", Router::new())
                    .nest("/ref/role/res", Router::new())
                    .nest("/ref/role/api", Router::new())
                    .nest("/ref/role/data", Router::new())
                    .nest("/ref/role/user", Router::new()),
            )
            .nest("/v2", Router::new()),
    )
}
