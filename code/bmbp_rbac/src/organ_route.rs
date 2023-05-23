use axum::extract::Path;
use axum::Json;
use tracing;

use bmbp_types::{BmbpError, BmbpResp, RespVo};

use crate::organ_model::{BmbpRbacOrgan, OrganQueryParam};
use crate::organ_service::OrganService;

pub async fn find_organ_tree(
    params: Json<OrganQueryParam>,
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

pub async fn find_organ_page() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn find_organ_page_by_parent() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn find_organ_list() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn find_organ_list_by_parent() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn find_organ_info_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn find_organ_info_by_code() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn save_organ(Json(mut organ): Json<BmbpRbacOrgan>) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    tracing::debug!("保存组织信息:{:#?}", organ);
    OrganService::save_organ(&mut organ).await?;
    Ok(RespVo::ok_msg_data("保存组织机构成功!".to_string(), organ))
}

pub async fn insert_organ() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn update_organ() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn update_organ_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn update_organ_parent() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn enable_organ_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn disable_organ_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn remove_organ_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}

pub async fn batch_remove_organ_by_id() -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    Err(BmbpError::api("接口未实现".to_string()))
}
