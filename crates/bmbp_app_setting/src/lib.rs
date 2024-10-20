use tracing;

mod dict;
mod region;
mod router;
mod vars;

pub use router::build_setting_router;

/// 设置路由
pub fn set_route_scope() {
    tracing::info!("初始化设置模块路由")
}
