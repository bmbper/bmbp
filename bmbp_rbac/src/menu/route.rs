use axum::Json;

use bmbp_types::{BmbpResp, RespVo};

use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};

pub async fn query_menu_tree(
    Json(param): Json<MenuQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpMenuVo>>> {
    tracing::info!("查询菜单树");
    let resp = RespVo::<Vec<BmbpMenuVo>>::ok_data(vec![]);
    Ok(resp)
}
