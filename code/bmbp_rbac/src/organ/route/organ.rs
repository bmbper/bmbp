use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde_json::Value;

use bmbp_types::{err::BmbpResp, PageInner, RespVo};

use crate::organ::service::OrganService;
use crate::organ::{BmbpOrganModel, BmbpOrganUnitsVo, PageQueryParam, QueryParam};

/// 查询组织机构树
pub async fn find_organ_tree(
    Json(param): Json<QueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpOrganModel>>> {
    tracing::info!("组织机构-查询树");
    let organ_tree_data = OrganService::find_organ_tree(&param).await?;
    let resp = RespVo::<Vec<BmbpOrganModel>>::ok_data(organ_tree_data);
    Ok(resp)
}

/// 查询组织分页
pub async fn find_organ_page(
    Json(mut params): Json<PageQueryParam>,
) -> BmbpResp<RespVo<PageInner<BmbpOrganModel>>> {
    tracing::info!("组织机构列表-查询分页");
    let page_inner = OrganService::find_organ_page(&mut params).await?;
    let resp = RespVo::<PageInner<BmbpOrganModel>>::ok_data(page_inner);
    Ok(resp)
}

/// 查询组织列表
pub async fn find_organ_list(
    Json(param): Json<QueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpOrganModel>>> {
    tracing::info!("组织机构-查询列表");
    let organ_tree_data = OrganService::find_organ_list(&param).await?;
    let resp = RespVo::<Vec<BmbpOrganModel>>::ok_option(organ_tree_data);
    Ok(resp)
}

/// 查询组织
pub async fn find_organ_info(Path(r_id): Path<String>) -> BmbpResp<RespVo<Option<BmbpOrganModel>>> {
    tracing::info!("组织机构详情-查询记录ID[{}]", r_id);
    let organ_model = OrganService::find_organ_info_by_rid(&r_id).await?;
    let resp = RespVo::ok_data(organ_model);
    Ok(resp)
}

/// 保存组织
pub async fn save_organ(Json(mut organ): Json<BmbpOrganModel>) -> BmbpResp<RespVo<BmbpOrganModel>> {
    tracing::info!("保存组织机构【{}】信息", organ.get_organ_title().clone());
    let organ_data = OrganService::save_organ(&mut organ).await?;
    let resp = RespVo::<BmbpOrganModel>::ok_data(organ_data);
    Ok(resp)
}

/// 新增组织
pub async fn insert_organ(Json(organ): Json<BmbpOrganModel>) -> BmbpResp<RespVo<BmbpOrganModel>> {
    tracing::info!("{:#?}", organ);
    let resp = RespVo::<BmbpOrganModel>::default();
    Ok(resp)
}

/// 更新组织
pub async fn update_organ(Json(value): Json<BmbpOrganModel>) -> BmbpResp<RespVo<BmbpOrganModel>> {
    tracing::info!("{:#?}", value);
    let resp = RespVo::<BmbpOrganModel>::default();
    Ok(resp)
}

/// 修改组织上级
pub async fn change_organ_parent(Json(param): Json<QueryParam>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("修改组织上级");
    let organ_tree_data = OrganService::change_organ_parent(&param).await?;
    let resp = RespVo::<usize>::ok_data(organ_tree_data);
    Ok(resp)
}

/// 删除组织
pub async fn delete_organ(Path(r_id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("删除组织:{}=>{}", "rId", r_id);
    let mut delete_params = QueryParam::new();
    delete_params.set_r_id(r_id);
    let row_count = OrganService::delete_organ(&delete_params).await?;
    Ok(RespVo::<usize>::ok_msg_data(
        format!("成功删除记录数:{}", row_count),
        row_count,
    ))
}
