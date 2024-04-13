use salvo::{handler, Request, Response};

use bmbp_app_common::{
    BmbpError, BmbpPageParam, HttpRespListVo, HttpRespPageVo, HttpRespVo, RespVo,
};
use bmbp_app_utils::is_empty_string;

use crate::organ::model::{BmbpRbacOrganTree, OrganQueryParams};
use crate::organ::service::BmbpRbacOrganService;

#[handler]
pub async fn find_organ_tree(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let params = req.parse_json::<OrganQueryParams>().await?;
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_tree(params).await?,
    ))
}
#[handler]
pub async fn find_organ_tree_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let organ_id = req.param::<String>("code");
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_tree_by_code(organ_id).await?,
    ))
}
#[handler]
pub async fn find_organ_tree_exclude_person(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let params = req.parse_json::<OrganQueryParams>().await?;
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_tree_exclude_by_person(params).await?,
    ))
}
#[handler]
pub async fn find_organ_tree_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let data_id = req.param::<String>("dataId");
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_tree_by_id(data_id).await?,
    ))
}

#[handler]
pub async fn find_organ_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpRbacOrganTree> {
    let params = req.parse_json::<BmbpPageParam<OrganQueryParams>>().await?;
    Ok(RespVo::ok_find_data(
        BmbpRbacOrganService::find_organ_page(params).await?,
    ))
}

#[handler]
pub async fn find_organ_list(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let params = req.parse_json::<OrganQueryParams>().await?;
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_list(params).await?,
    ))
}

#[handler]
pub async fn find_organ_info(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpRbacOrganTree> {
    let organ_id = req.param::<String>("dataId");
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::find_organ_by_id(organ_id.as_ref()).await?,
    ))
}
#[handler]
pub async fn find_organ_tree_exclude_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpRbacOrganTree> {
    let organ_id = req.param::<String>("dataId");
    Ok(RespVo::ok_find_option(
        BmbpRbacOrganService::query_organ_tree_exclude_by_id(organ_id).await?,
    ))
}

#[handler]
pub async fn save_organ(req: &mut Request, _res: &mut Response) -> HttpRespVo<BmbpRbacOrganTree> {
    let mut organ_params = req.parse_json::<BmbpRbacOrganTree>().await?;
    let organ_id = organ_params.get_data_id();
    let mut organ_info = BmbpRbacOrganService::find_organ_by_id(organ_id).await?;
    if organ_info.is_none() {
        BmbpRbacOrganService::insert_organ(&mut organ_params).await?;
    } else {
        BmbpRbacOrganService::update_organ(&mut organ_params).await?;
    }
    organ_info = BmbpRbacOrganService::find_organ_by_id(organ_params.get_data_id()).await?;
    Ok(RespVo::ok_save_option(organ_info))
}
#[handler]
pub async fn save_organ_parent(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let organ_id = req.param::<String>("dataId");
    if is_empty_string(organ_id.as_ref()) {
        return Err(BmbpError::service("请指定字典要变更的字典！"));
    }
    let params = req.parse_json::<OrganQueryParams>().await?;
    let parent_code = params.get_parent_code().clone().cloned();
    if is_empty_string(parent_code.as_ref()) {
        return Err(BmbpError::service("请指定字典上级字典"));
    }
    Ok(RespVo::ok_save_data(
        BmbpRbacOrganService::update_organ_parent(organ_id, parent_code).await?,
    ))
}

#[handler]
pub async fn insert_organ(req: &mut Request, _res: &Response) -> HttpRespVo<BmbpRbacOrganTree> {
    let mut organ_params = req.parse_json::<BmbpRbacOrganTree>().await?;
    let organ_id = organ_params.get_data_id().clone().cloned();
    BmbpRbacOrganService::insert_organ(&mut organ_params).await?;
    let organ_info = BmbpRbacOrganService::find_organ_by_id(organ_id.as_ref()).await?;
    Ok(RespVo::ok_save_option(organ_info))
}

#[handler]
pub async fn update_organ(req: &mut Request, _res: &Response) -> HttpRespVo<BmbpRbacOrganTree> {
    let mut organ_params = req.parse_json::<BmbpRbacOrganTree>().await?;
    let organ_id = organ_params.get_data_id().clone().cloned();
    BmbpRbacOrganService::update_organ(&mut organ_params).await?;
    let organ_info = BmbpRbacOrganService::find_organ_by_id(organ_id.as_ref()).await?;
    Ok(RespVo::ok_save_option(organ_info))
}

#[handler]
pub async fn disable_organ(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let organ_id = req.param::<String>("dataId");
    Ok(RespVo::ok_remove_data(
        BmbpRbacOrganService::disable_organ(organ_id).await?,
    ))
}

#[handler]
pub async fn enable_organ(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let organ_id = req.param::<String>("dataId");
    Ok(RespVo::ok_enable_data(
        BmbpRbacOrganService::enable_organ(organ_id).await?,
    ))
}

#[handler]
pub async fn remove_organ(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let organ_id = req.param::<String>("dataId");
    Ok(RespVo::ok_remove_data(
        BmbpRbacOrganService::delete_organ_by_id(organ_id).await?,
    ))
}
