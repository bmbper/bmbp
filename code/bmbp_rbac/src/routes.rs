use crate::menu;
use crate::organ::*;
use crate::role::*;
use crate::user::*;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn build_rbac_router() -> Router {
    tracing::info!("初始化权限管理模块路由......");
    Router::new().nest(
        "/v1",
        Router::new().nest(
            "/rbac",
            Router::new()
                .nest(
                    "/menu",
                    Router::new()
                        .route("/tree", post(menu::find_tree))
                        .route("/page", post(menu::find_page))
                        .route("/grid", post(menu::find_list))
                        .route("/info/rid/:r_id", get(menu::find_info_by_r_id))
                        .route(
                            "/update/parent/id/:id/:parent_id",
                            post(menu::update_parent),
                        )
                        .route("/save", post(menu::save))
                        .route("/delete/id/:r_id", post(menu::delete_menu_info_r_id))
                        .route("/delete/node/:organ_id", post(delete_organ_by_node_id))
                        .route("/delete/batch/id", post(delete_organ_by_id_vec))
                        .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                )
                .nest("/app", Router::new())
                .nest(
                    "/organ",
                    Router::new()
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
                        .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                )
                .nest(
                    "/user",
                    Router::new()
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
                        .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                )
                .nest(
                    "/role",
                    Router::new()
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
                        .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                )
                .nest(
                    "/privilege",
                    Router::new()
                        .nest(
                            "/api",
                            Router::new()
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
                                .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                        )
                        .nest(
                            "/res",
                            Router::new()
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
                                .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                        )
                        .nest(
                            "/data",
                            Router::new()
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
                                .route("/delete/batch/node", post(delete_organ_by_node_id_vec)),
                        ),
                ),
        ),
    )
}
