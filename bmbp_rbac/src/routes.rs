use axum::{
    routing::{get, post},
    Router,
};

use crate::organ::*;
use crate::user::find_user_info_by_id;

pub fn build_rbac_router() -> Router {
    tracing::info!("初始化权限管理模块路由......");
    let v_router = Router::new()
        .merge(build_v1_router())
        .merge(build_v2_router());
    let rbac_router = Router::new().nest("/rbac", v_router);
    rbac_router
}

fn build_v1_router() -> Router {
    let child_router = Router::new()
        .merge(build_v1_organ_router())
        .merge(build_v1_user_router())
        .merge(build_v1_role_router())
        .merge(build_v1_menu_router())
        .merge(build_v1_api_router())
        .merge(build_v1_data_router());
    let v1_router = Router::new().nest("/v1", child_router);
    v1_router
}

fn build_v1_organ_router() -> Router {
    let router = Router::new()
        .route("/tree", post(query_organ_tree))
        .route("/tree/parent/:id", post(query_organ_tree_by_parent_id))
        .route("/tree/node/:id", post(query_organ_tree_by_node_id))
        .route("/tree/path/:path", post(query_organ_tree_by_path))
        .route("/tree/:field_name/:field_value", post(query_organ_tree))
        .route("/page", post(query_organ_page))
        .route("/grid", post(query_organ_grid))
        .route("/grid/:field_name/:field_value", post(query_organ_grid))
        .route("/info", post(query_organ_info_by_params))
        .route(
            "/info/:field_name/:field_value",
            get(query_organ_info_by_path).post(query_organ_info_by_path),
        )
        .route(
            "/info/id/:id",
            get(query_organ_info_by_id).post(query_organ_info_by_id),
        )
        .route("/save", post(save_organ))
        .route("/update", post(update_organ))
        .route(
            "/update/:field_name/:field_value",
            post(update_organ_by_path),
        )
        .route("/update_by_param", post(update_organ_by_param))
        .route(
            "/delete/:field_name/:field_value",
            get(delete_organ_by_path),
        )
        .route("/delete", post(delete_organ_by_params));
    let v1_router = Router::new().nest("/organ", router);
    v1_router
}

fn build_v1_user_router() -> Router {
    let mut router = Router::new();
    router = router.route("info", get(find_user_info_by_params));
    router = router.route("info/id/:id", get(find_user_info_by_id));
    let v1_router = Router::new().nest("/user", router);
    v1_router
}

fn build_v1_role_router() -> Router {
    let router = Router::new();
    let v1_router = Router::new().nest("/role", router);
    v1_router
}

fn build_v1_menu_router() -> Router {
    let router = Router::new();
    let v1_router = Router::new().nest("/menu", router);
    v1_router
}

fn build_v1_api_router() -> Router {
    let router = Router::new();
    let v1_router = Router::new().nest("/api", router);
    v1_router
}

fn build_v1_data_router() -> Router {
    let router = Router::new();
    let v1_router = Router::new().nest("/data", router);
    v1_router
}

fn build_v2_router() -> Router {
    let child_router = Router::new();
    let v2_router = Router::new().nest("/v2", child_router);
    v2_router
}
