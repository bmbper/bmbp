use super::model::AppQueryParams;
use super::model::BmbpRbacApp;
use super::service::RbacAppService;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageVo;
use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::Request;
use salvo::Response;

/// 根据参数查询应用分页列表
#[handler]
pub async fn find_app_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpRbacApp>>> {
    let params = req.parse_json::<AppQueryParams>().await?;
    let rs = RbacAppService::find_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}
