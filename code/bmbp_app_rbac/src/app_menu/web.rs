use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageParams;
use bmbp_app_common::PageVo;
use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::Request;
use salvo::Response;

use super::service::MenuService;

#[handler]
pub async fn find_menu_tree(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let params = req.parse_json::<BmbpHashMap>().await?;
    let rs = MenuService::find_menu_tree(&params).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn find_menu_tree_start_with_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let record_id = req.param::<String>("id");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定菜单ID"));
    }

    let rs = MenuService::find_menu_tree_start_with_id(&record_id.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_menu_tree_with_out_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let mut with_out_menu_id = "".to_string();
    if let Some(id) = req.param::<String>("id") {
        with_out_menu_id = id.to_string();
    }
    let mut app_id = "".to_string();
    if let Some(id) = req.param::<String>("appId") {
        app_id = id.to_string();
    }
    let rs = MenuService::find_menu_tree_with_out_id(&app_id, &with_out_menu_id).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn find_menu_tree_start_with_code(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    let menu_code = req.param::<String>("code");
    if menu_code.is_none() {
        return Err(BmbpError::api("请指定菜单编码"));
    }

    let rs = MenuService::find_menu_tree_start_with_code(&menu_code.unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_menu_page(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    let params = req.parse_json::<PageParams<BmbpHashMap>>().await?;
    let rs = MenuService::find_menu_page(&params).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn find_menu_page_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<PageVo<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

#[handler]
pub async fn find_menu_list(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

#[handler]
pub async fn find_menu_list_by_parent(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<Vec<BmbpHashMap>>>> {
    Err(BmbpError::api("接口未实现"))
}

#[handler]
pub async fn find_menu_info_by_id(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = MenuService::find_menu_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn find_menu_info_by_code(
    req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    let menu_code = req.param::<String>("menuCode");
    if menu_code.is_none() {
        return Err(BmbpError::api("无效的菜单编码"));
    }
    let rs = MenuService::find_menu_by_menu_code(menu_code.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn save_menu(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<BmbpHashMap>> {
    let mut params = req.parse_json::<BmbpHashMap>().await?;
    let _ = MenuService::save_menu(&mut params).await?;
    Ok(RespVo::ok_data(params))
}
#[handler]
pub async fn insert_menu(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
#[handler]
pub async fn update_menu(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}
#[handler]
pub async fn update_menu_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpHashMap>>> {
    Err(BmbpError::api("接口未实现"))
}

#[handler]
pub async fn update_menu_parent(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("请指定待变更的主键"));
    }
    let menu_parent_code = req.param::<String>("parent");
    if menu_parent_code.is_none() {
        return Err(BmbpError::api("请指定待变更的上级菜单"));
    }
    let rs = MenuService::update_menu_parent_by_record_id(
        &record_id.unwrap(),
        &menu_parent_code.unwrap(),
    )
    .await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn enable_menu_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs = MenuService::update_menu_status(record_id.as_ref().unwrap(), &"0".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn disable_menu_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("无效的主键ID"));
    }
    let rs =
        MenuService::update_menu_status(record_id.as_ref().unwrap(), &"-1".to_string()).await?;
    Ok(RespVo::ok_data(rs))
}
#[handler]
pub async fn remove_menu(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}
#[handler]
pub async fn remove_menu_by_id(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<usize>> {
    let record_id = req.param::<String>("recordId");
    if record_id.is_none() {
        return Err(BmbpError::api("指定的记录无效"));
    }
    let rs = MenuService::remove_menu_by_id(record_id.as_ref().unwrap()).await?;
    Ok(RespVo::ok_data(rs))
}

#[handler]
pub async fn batch_remove_menu_by_id(
    _req: &mut Request,
    _res: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    Err(BmbpError::api("接口未实现"))
}