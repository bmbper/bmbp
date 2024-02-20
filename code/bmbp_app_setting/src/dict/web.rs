use salvo::{handler, Request, Response};
use bmbp_app_common::{BmbpResp, HttpRespListVo, HttpRespPageVo, HttpRespVo, PageVo, RespVo};
use crate::dict::model::{BmbpSettingDictOrmModel, DictQueryParams};


/// find_dict_tree 查询字典树
/// 接收JSON参数：
///  ```json
///     {
///         code: 节点编码
///         showLevel: 显示层级
///         status:  节点状态
///     }
///  ```
///
#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    tracing::info!("find_dict_tree params: {:#?}", params);
    Ok(RespVo::ok_data(vec![]))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpSettingDictOrmModel> {
    Ok(PageVo::ok_data(vec![]))
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