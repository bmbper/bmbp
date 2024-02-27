use salvo::{handler, Request, Response};
use tracing::info;
use bmbp_app_common::{BmbpPageParam, BmbpResp, HttpRespListVo, HttpRespPageVo, HttpRespVo, PageVo, RespVo};
use crate::dict::model::{BmbpSettingDictOrmModel, DictQueryParams};
use bmbp_rdbc_orm::{Query, RdbcModel, RdbcORM, RdbcPage};
use crate::dict::scripts::build_query_script;

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    info!("find_dict_tree params: {:#?}", params);
    // 拼接查询条件
    let query = build_query_script();
    match RdbcORM.await.select_list_by_query::<BmbpSettingDictOrmModel>(&query).await {
        Ok(dict_vec) => Ok(RespVo::ok_option(dict_vec)),
        Err(e) => Ok(RespVo::fail_msg(e.get_msg().as_str()))
    }
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<BmbpPageParam<DictQueryParams>>().await?;
    info!("find_dict_tree params: {:#?}", params);
    // 拼接查询条件
    let query = build_query_script();
    let mut page: RdbcPage<BmbpSettingDictOrmModel> = RdbcPage::new();
    page.set_page_num(params.get_page_no().clone()).set_page_size(params.get_page_size().clone());
    match RdbcORM.await.select_page_by_query::<BmbpSettingDictOrmModel>(&mut page, &query).await {
        Ok(page_) => {
            let rbac_page = PageVo::new_page(page_.page_num().clone(),
                                             page_.page_size().clone(), page_.total().clone(), page.data_take());
            Ok(RespVo::ok_data(rbac_page))
        }
        Err(e) => Ok(RespVo::fail_msg(e.get_msg().as_str()))
    }
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let dict = BmbpSettingDictOrmModel::default();
    Ok(RespVo::ok_option(None))
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
    let dict = BmbpSettingDictOrmModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let dict = BmbpSettingDictOrmModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let dict = BmbpSettingDictOrmModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
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