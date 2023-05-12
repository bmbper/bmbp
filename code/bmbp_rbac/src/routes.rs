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
    Router::new()
}
