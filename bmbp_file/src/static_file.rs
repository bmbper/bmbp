use std::io;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get_service, MethodRouter},
};
use tower_http::services::ServeDir;

/// build_static_method_serve  构建静态文件服务
pub fn build_static_method_serve() -> MethodRouter {
    async fn handle_error(err: io::Error) -> impl IntoResponse {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("静态文件解析内部错误:{:#?}", err),
        )
    }

    let serve_dir = ServeDir::new("webapp/lib");
    let serve_dir_router = get_service(serve_dir).handle_error(handle_error);
    serve_dir_router
}
