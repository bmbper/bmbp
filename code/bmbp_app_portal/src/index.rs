use axum::response::IntoResponse;

pub async fn index_api() -> impl IntoResponse {
    "欢迎使用bmbp应用模块，本项目仅提供后台接口，前端项目请移步bmbp_ui"
}
