use axum::Router;

use crate::organ::organ_router;
use crate::user::user_router;

pub fn build_rbac_router() -> Router {
    tracing::info!("初始化权限管理模块路由......");
    Router::new().nest(
        "/rbac",
        Router::new()
            .nest(
                "/v1",
                Router::new()
                    .nest("/organ", organ_router())
                    .nest("/user", user_router())
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
