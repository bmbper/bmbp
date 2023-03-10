use axum::extract::Path;
use axum::Json;

use crate::menu::service::MenuService;
use bmbp_types::{BmbpPageReqVo, BmbpResp, PageInner, RespVo};

use crate::menu::vopo::{AppQueryParam, BmbpAppVo, BmbpMenuVo, MenuQueryParam};

#[allow(unused)]
pub async fn find_app_page(
    Json(params): Json<AppQueryParam>,
) -> BmbpResp<RespVo<PageInner<BmbpAppVo>>> {
    tracing::info!("分页查询应用列表");
    Ok(RespVo::fail_msg("请求未实现".to_string()))
}
#[allow(unused)]
pub async fn find_app_list(Json(params): Json<AppQueryParam>) -> BmbpResp<RespVo<Vec<BmbpAppVo>>> {
    tracing::info!("查询应用列表");
    Ok(RespVo::fail_msg("请求未实现".to_string()))
}

#[allow(unused)]
pub async fn find_app_info_r_id(Path(r_id): Path<String>) -> BmbpResp<RespVo<BmbpAppVo>> {
    tracing::info!("查询应用详情");
    Ok(RespVo::fail_msg("请求未实现".to_string()))
}

#[allow(unused)]
pub async fn find_app_info_app_id(Path(app_id): Path<String>) -> BmbpResp<RespVo<BmbpAppVo>> {
    tracing::info!("查询应用详情");
    Ok(RespVo::fail_msg("请求未实现".to_string()))
}

#[allow(unused)]
pub async fn delete_app_info_r_id(Path(r_id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("通过记录主键删除应用");
    Ok(RespVo::fail_msg("接口为未实现".to_string()))
}

#[allow(unused)]
pub async fn delete_app_info_app_id(Path(app_id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("通过应用ID删除应用");
    Ok(RespVo::fail_msg("接口为未实现".to_string()))
}

#[allow(unused)]
pub async fn save_app(Json(app_vo): Json<BmbpMenuVo>) -> BmbpResp<RespVo<BmbpAppVo>> {
    tracing::info!("保存应用");
    Ok(RespVo::fail_msg("接口为未实现".to_string()))
}

pub async fn find_tree(
    Json(params): Json<MenuQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpMenuVo>>> {
    tracing::info!("查询菜单树");
    let mut tree_params = MenuQueryParam::default();
    tree_params.set_app_id(params.get_app_id().clone());
    let menu_tree = MenuService::find_tree(&tree_params).await?;
    Ok(RespVo::ok_data(menu_tree))
}

pub async fn find_tree_by_parent_id(
    Path(parent_menu_id): Path<String>
) -> BmbpResp<RespVo<Vec<BmbpMenuVo>>> {
    tracing::info!("查询菜单树");
    let mut tree_params = MenuQueryParam::default();
    tree_params.set_r_id(parent_menu_id);
    let menu_tree = MenuService::find_tree(&tree_params).await?;
    Ok(RespVo::ok_data(menu_tree))
}

pub async fn find_tree_by_node_id(
    Path(menu_id): Path<String>
) -> BmbpResp<RespVo<Vec<BmbpMenuVo>>> {
    tracing::info!("查询菜单树");
    let mut tree_params = MenuQueryParam::default();
    tree_params.set_app_id(menu_id);
    let menu_tree = MenuService::find_tree(&tree_params).await?;
    Ok(RespVo::ok_data(menu_tree))
}

#[allow(unused)]
pub async fn find_page(
    Json(params): Json<BmbpPageReqVo<MenuQueryParam>>,
) -> BmbpResp<RespVo<PageInner<BmbpMenuVo>>> {
    tracing::info!(
        "分页查询菜单列表:{}",
        serde_json::to_string(&params).unwrap()
    );
    let vo = MenuService::find_page(&params).await?;
    Ok(RespVo::ok_data(vo))
}
#[allow(unused)]
pub async fn find_list(
    Json(params): Json<MenuQueryParam>,
) -> BmbpResp<RespVo<Vec<BmbpMenuVo>>> {
    tracing::info!("查询菜单列表");
    let vo = MenuService::find_list(&params).await?;
    Ok(RespVo::ok_data(vo))
}

#[allow(unused)]
pub async fn find_info_by_r_id(Path(r_id): Path<String>) -> BmbpResp<RespVo<BmbpMenuVo>> {
    tracing::info!("查询菜单列表");
    let mut params = MenuQueryParam::default();
    params.set_r_id(r_id);
    let vo = MenuService::find_one(&params).await?;
    Ok(RespVo::ok_option(vo))
}

pub async fn update_parent(Path((id,parent_id)): Path<(String,String)>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("修改菜单上级:{}-{}",id,parent_id);
    let vo = MenuService::update_parent(id,parent_id).await?;
    Ok(RespVo::<usize>::ok_option(Some(vo)))
}

pub async fn find_info_by_id(Path(id): Path<String>) -> BmbpResp<RespVo<BmbpMenuVo>> {
    let mut params = MenuQueryParam::default();
    params.set_menu_id(id);
    let vo = MenuService::find_one(&params).await?;
    Ok(RespVo::ok_option(vo))
}

#[allow(unused)]
pub async fn delete_menu_info_r_id(Path(r_id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("根据记录ID删除菜单详情");
    let mut params = MenuQueryParam::default();
    params.set_r_id(r_id);
    let vo = MenuService::delete(&params).await?;
    Ok(RespVo::ok_data(vo))
}

#[allow(unused)]
pub async fn delete_menu_info_menu_id(Path(menu_id): Path<String>) -> BmbpResp<RespVo<usize>> {
    tracing::info!("根据菜单ID删除菜单详情");
    let mut params = MenuQueryParam::default();
    params.set_menu_id(menu_id);
    let vo = MenuService::delete(&params).await?;
    Ok(RespVo::ok_data(vo))
}

#[allow(unused)]
pub async fn save(Json(mut menu_vo): Json<BmbpMenuVo>) -> BmbpResp<RespVo<BmbpMenuVo>> {
    tracing::debug!("保存菜单:{:#?}", menu_vo);
    let _ = MenuService::save(&mut menu_vo).await?;
    Ok(RespVo::ok_data(menu_vo))
}
