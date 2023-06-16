use axum::extract::Path;
use axum::Json;
use tracing;

use bmbp_types::{
    BmbpError, BmbpResp, PageParams, PageRespVo, RespVo, RECORD_STATUS_DISABLE,
    RECORD_STATUS_ENABLE,
};

use crate::organ::model::{BmbpRbacOrgan, OrganQueryParam};
use crate::organ::service::OrganService;

pub async fn find_organ_tree(
    Json(params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织树查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_tree_start_with_id(
    Path(id): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let mut params = OrganQueryParam::default();
    params.set_r_id(id.clone());
    tracing::debug!("组织树查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_tree_start_with_code(
    Path(code): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let mut params = OrganQueryParam::default();
    params.set_organ_id(code.clone());
    tracing::debug!("组织树查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_tree_start_with_parent(
    Path(parent): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let mut params = OrganQueryParam::default();
    params.set_organ_parent_id(parent.clone());
    tracing::debug!("组织树查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_tree(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_page(
    Json(params): Json<PageParams<OrganQueryParam>>,
) -> BmbpResp<RespVo<PageRespVo<BmbpRbacOrgan>>> {
    tracing::debug!("组织分页查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_page(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_page_by_parent(
    Path(parent): Path<String>,
    Json(mut params): Json<PageParams<OrganQueryParam>>,
) -> BmbpResp<RespVo<PageRespVo<BmbpRbacOrgan>>> {
    params.get_mut_params().unwrap().set_organ_parent_id(parent);
    tracing::debug!("组织分页查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_page(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_list(
    Json(params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("组织列表查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_list(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_list_by_parent(
    Path(parent): Path<String>,
    Json(mut params): Json<OrganQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    params.set_organ_parent_id(parent);
    tracing::debug!("组织列表查询参数:{:#?}", params);
    let organ_tree = OrganService::find_organ_list(&params).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树成功!".to_string(),
        organ_tree,
    ))
}

pub async fn find_organ_info_by_id(
    Path(id): Path<String>,
) -> BmbpResp<RespVo<Option<BmbpRbacOrgan>>> {
    tracing::debug!("组织详情查询参数r_id:{:#?}", id);
    let organ = OrganService::find_organ_by_id(&id).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树详情!".to_string(),
        organ,
    ))
}

pub async fn find_organ_info_by_code(
    Path(code): Path<String>,
) -> BmbpResp<RespVo<Option<BmbpRbacOrgan>>> {
    tracing::debug!("组织详情查询参数organ_code:{:#?}", code);
    let organ = OrganService::find_organ_by_organ_code(&code).await?;
    Ok(RespVo::ok_msg_data(
        "查询组织机构树详情!".to_string(),
        organ,
    ))
}

pub async fn save_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("保存组织信息:{:#?}", organ);
    OrganService::save_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!".to_string(), organ))
}

pub async fn insert_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("新增组织信息:{:#?}", organ);
    OrganService::insert_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!".to_string(), organ))
}

pub async fn update_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("更新组织信息:{:#?}", organ);
    OrganService::update_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!".to_string(), organ))
}

pub async fn update_organ_by_id(
    Path(id): Path<String>,
    Json(mut organ): Json<BmbpRbacOrgan>,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    organ.set_r_id(id);
    tracing::debug!("更新组织信息:{:#?}", organ);
    OrganService::update_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!".to_string(), organ))
}

pub async fn update_organ_parent(
    Path(id): Path<String>,
    Path(parent): Path<String>,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    tracing::debug!("更新组织父级:{}:{}", id, parent);
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn enable_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("启用组织{}", id);
    let row_count = OrganService::update_organ_status(id, RECORD_STATUS_ENABLE.to_string()).await?;
    Ok(RespVo::ok_msg_data(
        "启用组织机构成功!".to_string(),
        row_count,
    ))
}

pub async fn disable_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("停用组织{}", id);
    let row_count =
        OrganService::update_organ_status(id, RECORD_STATUS_DISABLE.to_string()).await?;
    Ok(RespVo::ok_msg_data(
        "停用组织机构成功!".to_string(),
        row_count,
    ))
}

pub async fn remove_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("删除组织{}", id);
    let row_count = OrganService::remove_organ_by_id(id).await?;
    Ok(RespVo::ok_msg_data(
        "删除组织机构成功!".to_string(),
        row_count,
    ))
}

pub async fn batch_remove_organ_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::debug!("删除组织{}", id);
    let row_count = OrganService::remove_organ_by_id(id).await?;
    Ok(RespVo::ok_msg_data(
        "删除组织机构成功!".to_string(),
        row_count,
    ))
}
