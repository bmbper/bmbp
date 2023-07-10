use bmbp_app_common::PageVo;
use bmbp_app_common::RespVo;
use salvo::handler;
use salvo::writing::Json;
use salvo::Request;
use salvo::Response;

/// 根据参数查询组织机构树
#[handler]
pub async fn find_organ_tree(_req: &mut Request, _res: &mut Response) {
    _res.render("查询组织权树接kkkkk")
}

/// 查询指定REOCORD_ID开始的组织机构树
#[handler]
pub async fn find_organ_tree_start_with_id(req: &mut Request, res: &mut Response) {
    tracing::info!("查询组织构树{:#?}", req.param::<String>("id"));
    res.render(req.param::<String>("id").as_ref().unwrap())
}

/// 查询指定ORGAN_CODE开始的组织机构树
#[handler]
pub async fn find_organ_tree_start_with_code(_req: &mut Request, _res: &mut Response) {}
/// 分页查询组织机构列表
#[handler]
pub async fn find_organ_page(_req: &mut Request, res: &mut Response) {
    let resp: RespVo<PageVo<String>> = RespVo::ok_data(PageVo::default());
    res.render(Json(resp))
}

/// 分页查询指定ORGAN_PARENT_CODE组织下的机构列表
#[handler]
pub async fn find_organ_page_by_parent(_req: &mut Request, _res: &mut Response) {}

/// 查询组织机构列表
#[handler]
pub async fn find_organ_list(_req: &mut Request, res: &mut Response) {
    res.render("查询树列表")
}

/// 查询指定ORGAN_PARENT_CODE组织下的组织机构列表
#[handler]
pub async fn find_organ_list_by_parent(_req: &mut Request, _res: &mut Response) {}

/// 查询指定RECORD_ID组织机构详情
#[handler]
pub async fn find_organ_info_by_id(_req: &mut Request, _res: &mut Response) {}
/// 查询指定RECORD_CODE的组织机构详情
#[handler]
pub async fn find_organ_info_by_code(_req: &mut Request, _res: &mut Response) {}

/// 保存组织机构
#[handler]
pub async fn save_organ(_req: &mut Request, _res: &mut Response) {}
/// 插入组织机构
#[handler]
pub async fn insert_organ(_req: &mut Request, _res: &mut Response) {}
/// 更新组织机构
#[handler]
pub async fn update_organ(_req: &mut Request, _res: &mut Response) {}
/// 更新指定RECORD_ID的组织机构
#[handler]
pub async fn update_organ_by_id(_req: &mut Request, _res: &mut Response) {}

/// 更新组织机构的上级信息
#[handler]
pub async fn update_organ_parent(_req: &mut Request, _res: &mut Response) {}
/// 启用指定RECORD_ID的组织机构
#[handler]
pub async fn enable_organ_by_id(_req: &mut Request, _res: &mut Response) {}
/// 停用指定RECORD_ID的组织机构
#[handler]
pub async fn disable_organ_by_id(_req: &mut Request, _res: &mut Response) {}
/// 删除组织机构
#[handler]
pub async fn remove_organ(_req: &mut Request, _res: &mut Response) {}
/// 删除指定RECORD_ID的组织机构
#[handler]
pub async fn remove_organ_by_id(_req: &mut Request, _res: &mut Response) {}

/// 批量删除指定RECORD_ID的组织机构
#[handler]
pub async fn batch_remove_organ_by_id(_req: &mut Request, _res: &mut Response) {}
