use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::menu;
use crate::menu::{find_info_by_r_id, update_parent};
use crate::organ::*;
use crate::role::*;
use crate::user::*;

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
                        .route("/delete/id/:r_id", post(menu::delete_menu_info_r_id)),
                )
                .nest("/app", Router::new())
                .nest(
                    "/organ",
                    Router::new().nest(
                        "/organ",
                        Router::new()
                            .route("/tree", post(find_organ_tree))
                            .route("/page", post(find_organ_page))
                            .route("/list", post(find_organ_list))
                            .route("/info/{rId}", post(find_organ_info))
                            .route("/save", post(save_organ))
                            .route("/insert", post(insert_organ))
                            .route("/update", post(update_organ))
                            .route("/delete/{rId}", post(delete_organ))
                            .route("/change/parent", post(change_organ_parent)),
                    ),
                )
                .nest(
                    "/units",
                    Router::new().route("/info/{rId}", get(find_info_by_r_id)),
                )
                .nest("/user", Router::new())
                .nest("/role", Router::new())
                .nest(
                    "/privilege",
                    Router::new()
                        .nest("/api", Router::new())
                        .nest("/res", Router::new())
                        .nest("/data", Router::new()),
                ),
        ),
    )
}
