use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use bmbp_types::{BmbpResp, RespVo};

pub async fn find_role_info_by_id(Path(id): Path<String>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{}", id);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn find_role_info_by_params(Json(value): Json<Value>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn save_role(Json(value): Json<Value>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}
