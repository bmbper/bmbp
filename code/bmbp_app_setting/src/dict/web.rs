use salvo::{handler, Request, Response};
use serde_json::{Map, Value};

use bmbp_app_common::{
    BmbpError, BmbpPageParam, BmbpResp, HttpRespListVo, HttpRespPageVo, HttpRespVo, RespVo,
};
use bmbp_app_utils::is_empty_string;

use crate::dict::model::{BmbpComboVo, BmbpSettingDictOrmModel, DictQueryParams};
use crate::dict::service::BmbpRbacDictService;

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    Ok(RespVo::ok_option(
        BmbpRbacDictService::query_dict_tree(params).await?,
    ))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespPageVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<BmbpPageParam<DictQueryParams>>().await?;
    Ok(RespVo::ok_data(
        BmbpRbacDictService::query_dict_page(params).await?,
    ))
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let params = req.parse_json::<DictQueryParams>().await?;
    Ok(RespVo::ok_option(
        BmbpRbacDictService::query_dict_list(params).await?,
    ))
}

#[handler]
pub async fn find_dict_info(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let dict_id = req.param::<String>("dataId");
    Ok(RespVo::ok_option(
        BmbpRbacDictService::query_dict_by_id(dict_id.as_ref()).await?,
    ))
}

#[handler]
pub async fn save_dict(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id();
    let mut dict_info = BmbpRbacDictService::query_dict_by_id(dict_id).await?;
    if dict_info.is_none() {
        BmbpRbacDictService::insert_dict_info(&mut dict_params).await?;
    } else {
        BmbpRbacDictService::update_dict_info(&mut dict_params).await?;
    }
    dict_info = BmbpRbacDictService::query_dict_by_id(dict_params.get_data_id()).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    _res: &Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id().clone().cloned();
    BmbpRbacDictService::insert_dict_info(&mut dict_params).await?;
    let dict_info = BmbpRbacDictService::query_dict_by_id(dict_id.as_ref()).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    _res: &Response,
) -> HttpRespVo<BmbpSettingDictOrmModel> {
    let mut dict_params = req.parse_json::<BmbpSettingDictOrmModel>().await?;
    let dict_id = dict_params.get_data_id().clone().cloned();
    BmbpRbacDictService::update_dict_info(&mut dict_params).await?;
    let dict_info = BmbpRbacDictService::query_dict_by_id(dict_id.as_ref()).await?;
    Ok(RespVo::ok_option(dict_info))
}

#[handler]
pub async fn disable_dict(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let dict_id = req.param::<String>("dataId");
    Ok(RespVo::ok_data(
        BmbpRbacDictService::disable_dict_status(dict_id).await?,
    ))
}

#[handler]
pub async fn enable_dict(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let dict_id = req.param::<String>("dataId");
    Ok(RespVo::ok_data(
        BmbpRbacDictService::enable_dict_status(dict_id).await?,
    ))
}

#[handler]
pub async fn delete_dict(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let dict_id = req.param::<String>("dataId");
    Ok(RespVo::ok_data(
        BmbpRbacDictService::delete_dict_info(dict_id).await?,
    ))
}

#[handler]
pub async fn find_dict_tree_exclude_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpSettingDictOrmModel> {
    let dict_id = req.param::<String>("dataId");
    Ok(RespVo::ok_option(
        BmbpRbacDictService::query_dict_tree_exclude_by_id(dict_id).await?,
    ))
}

#[handler]
pub async fn save_dict_parent(req: &mut Request, _res: &mut Response) -> HttpRespVo<usize> {
    let dict_id = req.param::<String>("dataId");
    if is_empty_string(dict_id.as_ref()) {
        return Err(BmbpError::service("请指定字典要变更的字典！"));
    }
    let params = req.parse_json::<DictQueryParams>().await?;
    let parent_code = params.get_parent_code().clone().cloned();
    if is_empty_string(parent_code.as_ref()) {
        return Err(BmbpError::service("请指定字典上级字典"));
    }
    Ok(RespVo::ok_data(
        BmbpRbacDictService::update_dict_parent(dict_id, parent_code).await?,
    ))
}

#[handler]
pub async fn find_combo_by_alias(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_combo_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_combo_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_cascade_combo_by_alias(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_cascade_combo_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_cascade_combo_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespListVo<BmbpComboVo> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_translate_by_alias(req: &mut Request, _res: &mut Response) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_translate_by_code(req: &mut Request, _res: &mut Response) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_translate_by_id(req: &mut Request, _res: &mut Response) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}

#[handler]
pub async fn find_cascade_translate_by_alias(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_cascade_translate_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}
#[handler]
pub async fn find_cascade_translate_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> HttpRespVo<Value> {
    Ok(RespVo::ok_option(None))
}
