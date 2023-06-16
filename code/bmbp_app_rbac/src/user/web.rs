use super::model::{RbacUser, UserParams};
use axum::extract::Json;
use bmbp_types::BmbpError;
use bmbp_types::BmbpResp;
use bmbp_types::RespVo;
pub async fn find_user_tree(Json(params): Json<UserParams>) -> BmbpResp<RespVo<Vec<RbacUser>>> {
    tracing::debug!("用户树查询参数:{:#?}", params);
    Err(BmbpError::api("接口未实现".to_string()))
}
