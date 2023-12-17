use salvo::{handler, Request, Response};
use tracing::{debug};
use bmbp_app_common::{BmbpResp, PageVo, RespVo};
use crate::dict::model::{BmbpSettingDictOrmTreeModel, DictQueryParams};
use crate::dict::service::BmbpSettingDictService;

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
) -> BmbpResp<RespVo<Vec<BmbpSettingDictOrmTreeModel>>> {
    let params: DictQueryParams = req.parse_json().await?;
    debug!("find_dict_tree params: {}", serde_json::to_string_pretty(&params).unwrap_or("".to_string()));
    let dict_tree = BmbpSettingDictService::find_dict_tree(&params).await?;
    Ok(RespVo::ok_data(dict_tree))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<PageVo<BmbpSettingDictOrmTreeModel>> {
    Ok(PageVo::ok_data(vec![]))
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<BmbpSettingDictOrmTreeModel>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_dict_info(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn save_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn disable_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn enable_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn delete_dict(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_data(Some(dict_list)))
}

#[handler]
pub async fn find_dict_tree_exclude_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpSettingDictOrmTreeModel>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_data(dict_list))
}

#[handler]
pub async fn save_dict_parent(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDictOrmTreeModel>>>> {
    let dict = BmbpSettingDictOrmTreeModel::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_data(Some(dict_list)))
}