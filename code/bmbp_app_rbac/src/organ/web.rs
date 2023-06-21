use axum::extract::Path;
use axum::Json;
use tracing;

use bmbp_app_common::{
    BmbpError, BmbpResp, PageParams, PageVo, RespVo, RECORD_STATUS_DISABLE, RECORD_STATUS_ENABLE,
};

use crate::organ::model::{BmbpRbacOrgan, OrganQueryParam};

use crate::organ::service::OrganService;

/// 根据参数查询组织机构树
pub async fn find_organ_tree(
    Json(params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织树查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 查询指定REOCORD_ID开始的组织机构树
pub async fn find_organ_tree_start_with_id(
    Path(id): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织树查询参数 record_id:{:#?}", id);
    let organ_tree = OrganService::find_organ_tree_start_with_id(&id).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 查询指定ORGAN_CODE开始的组织机构树
pub async fn find_organ_tree_start_with_code(
    Path(code): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织树查询参数 organ_code:{:#?}", code);
    let organ_tree = OrganService::find_organ_tree_start_with_code(&code).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 分页查询组织机构列表
pub async fn find_organ_page(
    Json(params): Json<PageParams<OrganQueryParam>>,
) -> BmbpResp<RespVo<PageVo<BmbpRbacOrgan>>> {
    tracing::debug!("组织分页查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_page(&params).await?;
    Ok(RespVo::ok_msg_data("查询组织机构分页成功!", organ_tree))
}

/// 分页查询指定ORGAN_PARENT_CODE组织下的机构列表
pub async fn find_organ_page_by_parent(
    Path(parent): Path<String>,
    Json(mut params): Json<PageParams<OrganQueryParam>>,
) -> BmbpResp<RespVo<PageVo<BmbpRbacOrgan>>> {
    tracing::debug!("组织分页查询参数 organ_parent_code:{:#?}", params);
    let organ_tree = OrganService::find_organ_page_by_parent(&parent, &params).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 查询组织机构列表
pub async fn find_organ_list(
    Json(params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织列表查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_list(&params).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 查询指定ORGAN_PARENT_CODE组织下的组织机构列表
pub async fn find_organ_list_by_parent(
    Path(parent): Path<String>,
    Json(mut params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {

    tracing::debug!(
        "组织列表查询参数:organ_parent_code:{}, params: {:#?}",
        parent,
        params
    );
    let organ_tree = OrganService::find_organ_list(&params).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树成功!", organ_tree))
}

/// 查询指定RECORD_ID组织机构详情
pub async fn find_organ_info_by_id(
    Path(id): Path<String>,
) -> BmbpResp<RespVo<Option<BmbpRbacOrgan>>> {
    tracing::debug!("组织详情查询参数r_id:{:#?}", id);
    let organ = OrganService::find_organ_by_id(&id).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树详情!", organ))
}

/// 查询指定RECORD_CODE的组织机构详情
pub async fn find_organ_info_by_code(
    Path(code): Path<String>,
) -> BmbpResp<RespVo<Option<BmbpRbacOrgan>>> {
    tracing::debug!("组织详情查询参数organ_code:{:#?}", code);
    let organ = OrganService::find_organ_by_organ_code(&code).await?;
    Ok(RespVo::ok_msg_data("查询组织机构树详情!", organ))
}

/// 保存组织机构
pub async fn save_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("保存组织信息:{:#?}", organ);
    let _row_count = OrganService::save_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!", organ))
}

/// 插入组织机构
pub async fn insert_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("新增组织信息:{:#?}", organ);
    let _row_count = OrganService::insert_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("新增组织机构成功!", organ))
}

/// 更新组织机构
pub async fn update_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("更新组织信息:{:#?}", organ);
    let _row_count = OrganService::update_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("更新组织机构成功!", organ))
}

/// 更新指定RECORD_ID的组织机构
pub async fn update_organ_by_id(
    Path(id): Path<String>,
    Json(mut organ): Json<BmbpRbacOrgan>,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {

    tracing::debug!("更新组织信息 record_id:{} :{:#?}", id, organ);
    let _row_count = OrganService::update_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("更新组织机构成功!", organ))
}

/// 更新组织机构的上级信息
pub async fn update_organ_parent(
    Path(id): Path<String>,
    Path(parent): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("更新组织父级:{}:{}", id, parent);
    Err(BmbpError::api("接口未实现".to_string()))
}
/// 启用指定RECORD_ID的组织机构
pub async fn enable_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("启用组织{}", id);
    let row_count = OrganService::update_organ_status(id, RECORD_STATUS_ENABLE.to_string()).await?;
    Ok(RespVo::ok_msg_data("启用组织机构成功!", row_count))
}
/// 停用指定RECORD_ID的组织机构
pub async fn disable_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("停用组织{}", id);
    let row_count =
        OrganService::update_organ_status(id, RECORD_STATUS_DISABLE.to_string()).await?;
    Ok(RespVo::ok_msg_data("停用组织机构成功!", row_count))
}
/// 删除组织机构
pub async fn remove_organ(Json(params): Json<OrganQueryParam>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("删除组织:{:#?}", params);
    let row_count = OrganService::remove_organ(&params).await?;
    Ok(RespVo::ok_msg_data("删除组织机构成功!", row_count))
}
/// 删除指定RECORD_ID的组织机构
pub async fn remove_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("删除组织{}", id);
    let row_count = OrganService::remove_organ_by_id(id).await?;
    Ok(RespVo::ok_msg_data("删除组织机构成功!", row_count))
}

/// 批量删除指定RECORD_ID的组织机构
pub async fn batch_remove_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("删除组织{}", id);
    let row_count = OrganService::remove_organ_by_id(id).await?;
    Ok(RespVo::ok_msg_data("删除组织机构成功!", row_count))
}
