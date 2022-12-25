use axum::response::IntoResponse;

pub async fn index_api() -> impl IntoResponse {
    "欢迎访问Bmbp行业应用仓库"
}
