use axum::Json;

use bmbp_types::{BmbpResp, RespVo};

pub async fn query_menu_tree(Json(param): Json<QueryParam>) -> BmbpResp<RespVo<Vec<BmbpOrganVo>>> {
    tracing::info!("查询菜单树");
    let resp = RespVo::<Vec<BmbpOrganVo>>::ok_data(vec![]);
    Ok(resp)
}
