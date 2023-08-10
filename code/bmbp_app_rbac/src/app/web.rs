use super::service::RbacAppService;
use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageParams;
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
    let params = req.parse_json::<PageParams<BmbpHashMap>>().await?;
    let rs = RbacAppService::find_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_app_list(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpHashMap>>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let rs = RbacAppService::find_list(&params).await?;
    Ok(RespVo::ok_option(rs))
}

#[handler]
pub async fn find_app_info(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RbacAppService::find_info(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_option(rs))
}

#[handler]
pub async fn save_app(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<BmbpHashMap>> {
    let mut params = req.parse_json::<BmbpHashMap>().await?;
    let rs = RbacAppService::save_app(&mut params).await?;
    Ok(RespVo::ok_data(rs.clone()))
}

#[handler]
pub async fn disable_app(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RbacAppService::update_status(record_id.as_ref().unwrap(), "2".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn batch_disable_app(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    return Err(BmbpError::api("批量下线接口未实现"));
}

#[handler]
pub async fn restart_app(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RbacAppService::update_status(record_id.as_ref().unwrap(), "0".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn enable_app(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RbacAppService::update_status(record_id.as_ref().unwrap(), "1".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn batch_enable_app(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    return Err(BmbpError::api("批量发布接口未实现"));
}
#[handler]
pub async fn delete_app(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RbacAppService::delete_app(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn batch_delete_app(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    return Err(BmbpError::api("删除接口未实现"));
}
