use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageParams;
use bmbp_app_common::PageVo;
use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::Request;
use salvo::Response;

use super::service::RoleService;

/// 根据参数查询角色机构树
#[handler]
pub async fn find_role_tree(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let rs = RoleService::find_role_tree(&params).await?;
    Ok(RespVo::ok_data(rs))
}
/// 查询指定REOCORD_ID开始的角色机构树
#[handler]
pub async fn find_role_tree_start_with_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let record_id = req.param::<String>("id");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定角色ID"));
    }

    let rs = RoleService::find_role_tree_start_with_id(&record_id.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_role_tree_with_out_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    // 要排除的角色ID
    let mut with_out_role_id = "".to_string();
    if let Some(id) = req.param::<String>("id") {
        with_out_role_id = id.to_string();
    }
    let rs = RoleService::find_role_tree_with_out_id(&with_out_role_id).await?;
    Ok(RespVo::ok_data(rs))
}
/// 查询指定ORGAN_CODE开始的角色机构树
#[handler]
pub async fn find_role_tree_start_with_code(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let role_code = req.param::<String>("code");
    if role_code.is_none() {
        return Err(BmbpError::api("请指定角色编码"));
    }

    let rs = RoleService::find_role_tree_start_with_code(&role_code.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 分页查询角色机构列表
#[handler]
pub async fn find_role_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<PageParams<BmbpHashMap>>().await?;
    let rs = RoleService::find_role_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

/// 分页查询指定ORGAN_PARENT_CODE角色下的机构列表
#[handler]
pub async fn find_role_page_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询角色机构列表
#[handler]
pub async fn find_role_list(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询指定ORGAN_PARENT_CODE角色下的角色机构列表
#[handler]
pub async fn find_role_list_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询指定RECORD_ID角色机构详情
#[handler]
pub async fn find_role_info_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RoleService::find_role_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 查询指定RECORD_CODE的角色机构详情
#[handler]
pub async fn find_role_info_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let role_code = req.param::<String>("roleCode");
    if role_code.is_none() {
        return Err(BmbpError::api("无效的角色编码"));
    }
    let rs = RoleService::find_role_by_role_code(role_code.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

/// 保存角色机构
#[handler]
pub async fn save_role(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<BmbpHashMap>> {
    let mut params = req.parse_json::<BmbpHashMap>().await?;
    let _ = RoleService::save_role(&mut params).await?;
    Ok(RespVo::ok_data(params))
}
/// 插入角色机构
#[handler]
pub async fn insert_role(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新角色机构
#[handler]
pub async fn update_role(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新指定RECORD_ID的角色机构
#[handler]
pub async fn update_role_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 更新角色机构的上级信息
#[handler]
pub async fn update_role_parent(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定待变更的主键"));
    }
    let role_parent_code = req.param::<String>("parent");
    if role_parent_code.is_none() {
        return Err(BmbpError::api("请指定待变更的上级角色"));
    }
    let rs = RoleService::update_role_parent_by_record_id(
        &record_id.unwrap(),
        &role_parent_code.unwrap(),
    )
    .await?;
    Ok(RespVo::ok_data(rs))
}
/// 启用指定RECORD_ID的角色机构
#[handler]
pub async fn enable_role_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = RoleService::update_role_status(record_id.as_ref().unwrap(), &"0".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 停用指定RECORD_ID的角色机构
#[handler]
pub async fn disable_role_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs =
        RoleService::update_role_status(record_id.as_ref().unwrap(), &"-1".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 删除角色机构
#[handler]
pub async fn remove_role(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 删除指定RECORD_ID的角色机构
#[handler]
pub async fn remove_role_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("指定的记录无效"));
    }
    let rs = RoleService::remove_role_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

/// 批量删除指定RECORD_ID的角色机构
#[handler]
pub async fn batch_remove_role_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
