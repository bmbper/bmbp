use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageParams;
use bmbp_app_common::PageVo;
use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::Request;
use salvo::Response;

use super::service::OrganService;
use super::BmbpRbacOrgan;

/// 根据参数查询组织机构树
#[handler]
pub async fn find_organ_tree(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let rs = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_data(rs))
}
/// 查询指定REOCORD_ID开始的组织机构树
#[handler]
pub async fn find_organ_tree_start_with_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let record_id = req.param::<String>("id");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定组织ID"));
    }

    let rs = OrganService::find_organ_tree_start_with_id(&record_id.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

/// 查询指定ORGAN_CODE开始的组织机构树
#[handler]
pub async fn find_organ_tree_start_with_code(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let organ_code = req.param::<String>("code");
    if organ_code.is_none() {
        return Err(BmbpError::api("请指定组织编码c}"));
    }

    let rs = OrganService::find_organ_tree_start_with_code(&organ_code.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}
/// 分页查询组织机构列表
#[handler]
pub async fn find_organ_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<PageParams<BmbpHashMap>>().await?;
    let rs = OrganService::find_organ_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

/// 分页查询指定ORGAN_PARENT_CODE组织下的机构列表
#[handler]
pub async fn find_organ_page_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询组织机构列表
#[handler]
pub async fn find_organ_list(
    _req: &mut Request,
    res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询指定ORGAN_PARENT_CODE组织下的组织机构列表
#[handler]
pub async fn find_organ_list_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 查询指定RECORD_ID组织机构详情
#[handler]
pub async fn find_organ_info_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 查询指定RECORD_CODE的组织机构详情
#[handler]
pub async fn find_organ_info_by_code(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 保存组织机构
#[handler]
pub async fn save_organ(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 插入组织机构
#[handler]
pub async fn insert_organ(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新组织机构
#[handler]
pub async fn update_organ(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 更新指定RECORD_ID的组织机构
#[handler]
pub async fn update_organ_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 更新组织机构的上级信息
#[handler]
pub async fn update_organ_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 启用指定RECORD_ID的组织机构
#[handler]
pub async fn enable_organ_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 停用指定RECORD_ID的组织机构
#[handler]
pub async fn disable_organ_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 删除组织机构
#[handler]
pub async fn remove_organ(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
/// 删除指定RECORD_ID的组织机构
#[handler]
pub async fn remove_organ_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}

/// 批量删除指定RECORD_ID的组织机构
#[handler]
pub async fn batch_remove_organ_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
