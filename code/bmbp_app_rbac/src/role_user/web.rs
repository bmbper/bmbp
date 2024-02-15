use bmbp_app_common::{BmbpError, BmbpHashMap, BmbpResp, BmbpValue, BmbpPageParam, PageVo, RespVo};
use salvo::{handler, Request, Response};

use super::service::RbacRoleUserService;

#[handler]
pub async fn find_role_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<BmbpPageParam<BmbpHashMap>>().await?;
    let rs = RbacRoleUserService::find_role_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn find_role_tree_checked(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpValue>>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let rs = RbacRoleUserService::find_role_tree_checked(&params).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn remove_role_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("指定的记录无效"));
    }
    let rs = RbacRoleUserService::remove_role_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn add_user_roles(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpHashMap>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let _ = RbacRoleUserService::add_user_role(&params).await?;
    Ok(RespVo::ok_data(params))
}
