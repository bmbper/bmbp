use bmbp_app_common::{BmbpError, BmbpHashMap, BmbpResp, PageParams, PageVo, RespVo};
use salvo::{handler, Request, Response};

use super::service::UserService;

#[handler]
pub async fn find_user_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<PageParams<BmbpHashMap>>().await?;
    let rs = UserService::find_user_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

/// 查询指定RECORD_ID组织机构详情
#[handler]
pub async fn find_user_info_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = UserService::find_user_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 查询指定RECORD_CODE的组织机构详情
#[handler]
pub async fn find_user_info_by_name(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let organ_code = req.param::<String>("userName");
    if organ_code.is_none() {
        return Err(BmbpError::api("用户名称"));
    }
    let rs = UserService::find_user_by_user_name(organ_code.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn reset_user_password(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定重置密码的用户"));
    }
    let row_count = UserService::reset_user_password(&record_id.unwrap()).await?;
    Ok(RespVo::ok_data(row_count))
}
#[handler]
pub async fn save_user(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<BmbpHashMap>> {
    let mut params = req.parse_json::<BmbpHashMap>().await?;
    let _ = UserService::save_user(&mut params).await?;
    Ok(RespVo::ok_data(params))
}
/// 插入组织机构
#[handler]
pub async fn insert_user(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新组织机构
#[handler]
pub async fn update_user(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新指定RECORD_ID的组织机构
#[handler]
pub async fn update_user_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 更新组织机构的上级信息
#[handler]
pub async fn update_user_organ(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定待变更的主键"));
    }
    let user_parent_code = req.param::<String>("organCode");
    if user_parent_code.is_none() {
        return Err(BmbpError::api("请指定变更的组织"));
    }
    let rs = UserService::update_user_organ_by_record_id(
        &record_id.unwrap(),
        &user_parent_code.unwrap(),
    )
    .await?;
    Ok(RespVo::ok_data(rs))
}
/// 启用指定RECORD_ID的组织机构
#[handler]
pub async fn enable_user_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = UserService::update_user_status(record_id.as_ref().unwrap(), &"0".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 停用指定RECORD_ID的组织机构
#[handler]
pub async fn disable_user_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs =
        UserService::update_user_status(record_id.as_ref().unwrap(), &"-1".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 删除组织机构
#[handler]
pub async fn remove_user(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 删除指定RECORD_ID的组织机构
#[handler]
pub async fn remove_user_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("指定的记录无效"));
    }
    let rs = UserService::remove_user_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

/// 批量删除指定RECORD_ID的组织机构
#[handler]
pub async fn batch_remove_user_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
