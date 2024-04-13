use salvo::{handler, Request, Response};

use bmbp_app_common::{BmbpPageParam, HttpRespListVo, HttpRespPageVo, HttpRespVo, RespVo};

use crate::role::model::{BmbpRbacRoleModel, RbacRoleQueryParams};
use crate::role::service::BmbpRbacRoleService;

#[handler]
pub async fn find_role_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpRbacRoleModel> {
    let params = req
        .parse_json::<BmbpPageParam<RbacRoleQueryParams>>()
        .await?;
    Ok(RespVo::ok_find_data(
        BmbpRbacRoleService::find_role_page(params).await?,
    ))
}
#[handler]
pub async fn find_role_list(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacRoleModel> {
    let params = req.parse_json::<RbacRoleQueryParams>().await?;
    Ok(RespVo::ok_find_option(
        BmbpRbacRoleService::find_role_list(params).await?,
    ))
}
#[handler]
pub async fn find_role_info(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpRbacRoleModel> {
    let role_id = req.param::<String>("dataId");
    Ok(RespVo::ok_find_option(
        BmbpRbacRoleService::find_role_by_id(role_id.as_ref()).await?,
    ))
}

#[handler]
pub async fn save_role(req: &mut Request, _res: &mut Response) -> HttpRespVo<BmbpRbacRoleModel> {
    let mut role_params = req.parse_json::<BmbpRbacRoleModel>().await?;
    let role_id = role_params.get_data_id();
    let mut role_info = BmbpRbacRoleService::find_role_by_id(role_id).await?;
    if role_info.is_none() {
        BmbpRbacRoleService::insert_role(&mut role_params).await?;
    } else {
        BmbpRbacRoleService::update_role(&mut role_params).await?;
    }
    role_info = BmbpRbacRoleService::find_role_by_id(role_params.get_data_id()).await?;
    Ok(RespVo::ok_save_option(role_info))
}

#[handler]
pub async fn insert_role(req: &mut Request, _res: &Response) -> HttpRespVo<BmbpRbacRoleModel> {
    let mut role_params = req.parse_json::<BmbpRbacRoleModel>().await?;
    let role_id = role_params.get_data_id().clone().cloned();
    BmbpRbacRoleService::insert_role(&mut role_params).await?;
    let role_info = BmbpRbacRoleService::find_role_by_id(role_id.as_ref()).await?;
    Ok(RespVo::ok_save_option(role_info))
}

#[handler]
pub async fn update_role(req: &mut Request, _res: &Response) -> HttpRespVo<BmbpRbacRoleModel> {
    let mut role_params = req.parse_json::<BmbpRbacRoleModel>().await?;
    let role_id = role_params.get_data_id().clone().cloned();
    BmbpRbacRoleService::update_role(&mut role_params).await?;
    let role_info = BmbpRbacRoleService::find_role_by_id(role_id.as_ref()).await?;
    Ok(RespVo::ok_save_option(role_info))
}

#[handler]
pub async fn disable_role(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let role_id = req.param::<String>("dataId");
    Ok(RespVo::ok_remove_data(
        BmbpRbacRoleService::disable_role(role_id).await?,
    ))
}

#[handler]
pub async fn enable_role(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let role_id = req.param::<String>("dataId");
    Ok(RespVo::ok_enable_data(
        BmbpRbacRoleService::enable_role(role_id).await?,
    ))
}

#[handler]
pub async fn remove_role(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let role_id = req.param::<String>("dataId");
    Ok(RespVo::ok_remove_data(
        BmbpRbacRoleService::delete_role_by_id(role_id).await?,
    ))
}
