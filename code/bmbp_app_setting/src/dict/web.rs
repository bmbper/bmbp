use salvo::{handler, Request, Response};
use tracing::info;
use bmbp_app_common::{BmbpPageParam, BmbpResp, HttpRespListVo, HttpRespPageVo, HttpRespVo, PageVo, RespVo};
use bmbp_app_utils::{is_empty_string};
use crate::dict::model::{BmbpSettingDictOrmModel, DictQueryParams};
use bmbp_rdbc_orm::{Query, RdbcModel, RdbcORM, RdbcPage};
use crate::dict::scripts::build_query_script;
use crate::dict::service::{insert_dict_info, query_dict_by_id, query_dict_list, query_dict_page, query_dict_tree, update_dict_info};

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    info!("find_dict_tree params: {:#?}", params);
    // 拼接查询条件
    Ok(RespVo::ok_option(query_dict_tree(params).await?))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<BmbpPageParam<DictQueryParams>>().await?;
    info!("find_dict_tree params: {:#?}", params);
    Ok(RespVo::ok_data(query_dict_page(params).await?))
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    info!("find_dict_tree params: {:#?}", params);
    // 拼接查询条件
    Ok(RespVo::ok_option(query_dict_list(params).await?))
}

#[handler]
pub async fn find_dict_info(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let dict = BmbpSettingDictOrmModel::default();
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn save_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id().clone().cloned();
    let mut dict_info = query_dict_by_id(dict_id.clone()).await?;
    if dict_info.is_none() {
        insert_dict_info(dict_params).await?;
    } else {
        update_dict_info(dict_params).await?;
    }
    dict_info = query_dict_by_id(dict_id).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id().clone().cloned();
    insert_dict_info(dict_params).await?;
    let dict_info = query_dict_by_id(dict_id).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id().clone().cloned();
    update_dict_info(dict_params).await?;
    let dict_info = query_dict_by_id(dict_id).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn disable_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<usize> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn enable_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<usize> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn delete_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<usize> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_dict_tree_exclude_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    Ok(RespVo::ok_data(vec![]))
}

#[handler]
pub async fn save_dict_parent(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<usize> {
    Ok(RespVo::ok_option(None))
}