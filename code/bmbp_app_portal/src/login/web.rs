use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::Request;
use salvo::Response;

use crate::login::model::LoginUser;
use bmbp_app_common::BmbpResp;

use super::model::BmbpUserInfo;
use super::service::LoginService;

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> BmbpResp<RespVo<BmbpUserInfo>> {
    let login_user = req.parse_json::<LoginUser>().await.unwrap();
    let data = LoginService::do_login(&login_user).await?;
    Ok(RespVo::ok_data(data))
}
#[handler]
pub async fn login_out(_req: &mut Request, res: &mut Response) {
    res.render("logout")
}
