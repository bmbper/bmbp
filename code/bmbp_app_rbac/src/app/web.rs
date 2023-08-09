use super::model::AppQueryParams;
use super::model::BmbpRbacApp;
use super::service::RbacAppService;
use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
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
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<AppQueryParams>().await?;
    let rs = RbacAppService::find_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_app_list(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpHashMap>>> {
    let params = req.parse_json::<BmbpRbacApp>().await?;
    let rs = RbacAppService::find_list(&params).await?;
    Ok(RespVo::ok_option(rs))
}

#[handler]
pub async fn find_app_info(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    let record_id = req.param::<String>("recordId").as_ref();
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID".to_string()));
    }
    let rs = RbacAppService::find_info(record_id.unwrap()).await?;
    Ok(RespVo::ok_option(rs))
}
