use crate::login::model::LoginModel;
use axum::Json;
use bmbp_app_common::BmbpResp;

pub async fn login(Json(login): Json<LoginModel>) -> BmbpResp<()> {
    tracing::info!("登录信息：{:#?}", login);
    Ok(())
}

pub async fn login_out(Json(login): Json<LoginModel>) -> BmbpResp<()> {
    tracing::info!("登出信息：{:#?}", login);
    Ok(())
}
