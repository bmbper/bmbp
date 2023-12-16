use salvo::{handler, Request, Response};
use bmbp_app_common::{BmbpHashMap, BmbpResp, RespVo};
use crate::dict::model::BmbpSettingDict;

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpSettingDict>>>> {
    let dict = BmbpSettingDict::default();
    let dict_list = vec![dict];
    Ok(RespVo::ok_data(Some(dict_list)))
}