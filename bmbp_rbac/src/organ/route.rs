use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde_json::Value;

use bmbp_types::{err::BmbpResp, PageInner, RespVo};

use crate::organ::service::OrganService;
use crate::organ::vopo::{
    BmbpOrganDeptVo, BmbpOrganPersonVo, BmbpOrganPostVo, BmbpOrganUnitVo, BmbpOrganUnitsVo,
};

use super::vopo::{BmbpOrganVo, PageQueryParam, QueryParam};

pub async fn find_organ_tree(Json(param): Json<QueryParam>) -> BmbpResp<RespVo<Vec<BmbpOrganVo>>> {
    tracing::info!("查询组织机构树");
    let organ_tree_data = OrganService::find_organ_tree(&param).await?;
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(organ_tree_data);
    Ok(resp)
}

pub async fn find_organ_tree_by_parent(Path(id): Path<String>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("根据上级查询组织类型....");
    let mut query_param = QueryParam::default();
    query_param.set_parent_organ_id(id);
    let organ_tree_data = OrganService::find_organ_tree_by_parent(&mut query_param).await?;
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(organ_tree_data);
    Ok(resp)
}

pub async fn find_organ_tree_by_node(Path(id): Path<String>) -> BmbpResp<impl IntoResponse> {
    let mut param = QueryParam::default();
    param.set_organ_id(id);
    let organ_tree_data = OrganService::find_organ_tree_by_organ_id(&mut param).await?;
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(organ_tree_data);
    Ok(resp)
}

pub async fn find_organ_tree_by_node_path(
    Json(mut param): Json<QueryParam>,
) -> BmbpResp<impl IntoResponse> {
    if param.get_organ_path().is_empty() {
        return Ok(RespVo::fail_msg("请传入组织路径".to_string()));
    }
    let organ_tree_data = OrganService::find_organ_tree_by_organ_path(&param).await?;
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(organ_tree_data);
    Ok(resp)
}

pub async fn find_organ_page(
    Json(mut query_params): Json<PageQueryParam>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("组织机构列表-分页查询");
    let page_inner = OrganService::find_organ_page(&mut query_params).await?;
    let resp = RespVo::<PageInner<BmbpOrganVo>>::ok_data(page_inner);
    Ok(resp)
}

///  query_organ_grid 查询组织机构分页数据
pub async fn query_organ_grid(Json(param): Json<QueryParam>) -> BmbpResp<impl IntoResponse> {
    let mut param = QueryParam::default();
    param.set_organ_path("".to_string());
    let organ_tree_data = OrganService::find_organ_list(&param).await?;
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(organ_tree_data);
    Ok(resp)
}

pub async fn query_organ_info_by_id(Path(id): Path<String>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{}", id);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn query_organ_info_by_path(
    Path((field_name, field_value)): Path<(String, String)>,
) -> BmbpResp<impl IntoResponse> {
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn query_organ_info_by_params(Json(value): Json<Value>) -> BmbpResp<impl IntoResponse> {
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn save_organ(Json(mut value): Json<BmbpOrganVo>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("保存组织机构【{}】信息", value.get_organ_title().clone());
    let organ_data = OrganService::save_organ(&mut value).await?;
    let resp = RespVo::<BmbpOrganVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn save_organ_units(
    Json(mut value): Json<BmbpOrganUnitsVo>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let organ_data = OrganService::save_organ_units(&mut value).await?;
    let resp = RespVo::<BmbpOrganUnitsVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn save_organ_unit(
    Json(mut value): Json<BmbpOrganUnitVo>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("保存单位信息{:#?}", value);
    let organ_data = OrganService::save_organ_unit(&mut value).await?;
    let resp = RespVo::<BmbpOrganUnitVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn save_organ_dept(
    Json(mut value): Json<BmbpOrganDeptVo>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let organ_data = OrganService::save_organ_dept(&mut value).await?;
    let resp = RespVo::<BmbpOrganDeptVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn save_organ_post(
    Json(mut value): Json<BmbpOrganPostVo>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let organ_data = OrganService::save_organ_post(&mut value).await?;
    let resp = RespVo::<BmbpOrganPostVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn save_organ_person(
    Json(mut value): Json<BmbpOrganPersonVo>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let organ_data = OrganService::save_organ_person(&mut value).await?;
    let resp = RespVo::<BmbpOrganPersonVo>::ok_data(organ_data);
    Ok(resp)
}

pub async fn update_organ(Json(value): Json<Value>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn update_organ_by_path(
    Path((field_name, field_value)): Path<(String, String)>,
    Json(value): Json<Value>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    tracing::info!("{}：{}", field_name, field_value);

    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn update_organ_by_param(
    Query(param): Query<QueryParam>,
    Json(value): Json<Value>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    tracing::info!("{:#?}", param);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn delete_organ_by_path(
    Path((field_name, field_value)): Path<(String, String)>,
) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{}:{}", field_name, field_value);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}

pub async fn delete_organ_by_params(Json(value): Json<Value>) -> BmbpResp<impl IntoResponse> {
    tracing::info!("{:#?}", value);
    let resp = RespVo::<Value>::default();
    Ok(resp)
}
