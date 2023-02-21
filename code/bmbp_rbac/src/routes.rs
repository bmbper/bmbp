use crate::menu::*;
use crate::organ::*;
use crate::role::*;
use crate::user::*;
use axum::{
    routing::{delete, get, post},
    Router,
};

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
        .route("/tree", post(find_organ_tree))
        .route("/tree/parent/:id", post(find_organ_tree_by_parent))
        .route("/tree/node/:id", post(find_organ_tree_by_node))
        .route("/tree/path", post(find_organ_tree_by_node_path))
        .route("/page", post(find_organ_page))
        .route("/grid", post(query_organ_grid))
        .route("/info", post(query_organ_info_by_params))
        .route(
            "/info/id/:id",
            get(query_organ_info_by_r_id).post(query_organ_info_by_r_id),
        )
        .route("/info/organ/:id", get(query_organ_info_by_organ_id))
        .route("/save", post(save_organ))
        .route("/save/units", post(save_organ_units))
        .route("/save/unit", post(save_organ_unit))
        .route("/save/dept", post(save_organ_dept))
        .route("/save/post", post(save_organ_post))
        .route("/save/person", post(save_organ_person))
        .route("/update", post(update_organ))
        .route(
            "/update/:field_name/:field_value",
            post(update_organ_by_path),
        )
        .route("/update/parent", post(update_organ_parent))
        .route("/update_by_param", post(update_organ_by_param))
        .route("/delete/id/:r_id", post(delete_organ_by_id))
        .route("/delete/node/:organ_id", post(delete_organ_by_node_id))
        .route("/delete/batch/id", post(delete_organ_by_id_vec))
        .route("/delete/batch/node", post(delete_organ_by_node_id_vec));

    let v1_router = Router::new().nest("/organ", router);
    v1_router
}

fn build_v1_user_router() -> Router {
    let mut router = Router::new();
    router = router.route("/info", get(find_user_info_by_params));
    router = router.route("/info/id/:id", get(find_user_info_by_id));
    let v1_router = Router::new().nest("/user", router);
    v1_router
}

fn build_v1_role_router() -> Router {
    let mut router = Router::new();
    router = router
        .route("/info", get(find_role_info_by_params))
        .route("/info/id/:id", get(find_role_info_by_id))
        .route("/save", post(save_role));
    let v1_router = Router::new().nest("/role", router);
    v1_router
}

fn build_v1_menu_router() -> Router {
    let mut v1_router = Router::new();
    v1_router = v1_router.nest(
        "/app",
        Router::new()
            .route("/page", post(find_app_page))
            .route("/list", post(find_app_list))
            .route("/info/id/:r_id", get(find_app_info_r_id))
            .route("/info/app/:app_id", get(find_app_info_app_id))
            .route("/delete/id/:r_id", delete(delete_app_info_r_id))
            .route("/delete/app/:app_id", delete(delete_app_info_app_id))
            .route("/save", post(save_app)),
    );
    v1_router = v1_router.nest(
        "/menu",
        Router::new()
            .route("/tree", post(find_menu_tree))
            .route("/page", post(find_menu_page))
            .route("/list", post(find_menu_list))
            .route("/info/id/:r_id", get(find_menu_info_r_id))
            .route("/info/menu/:menu_id", get(find_menu_info_menu_id))
            .route("/delete/id/:r_id", delete(delete_menu_info_r_id))
            .route("/delete/menu/:app_id", delete(delete_menu_info_menu_id))
            .route("/save", post(save_menu)),
    );
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
