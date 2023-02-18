use crate::menu::dao::MenuDao;
use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};
use crate::util::{append_create_vo, append_update_vo};
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BmbpError, BmbpResp, PageInner, PageReqVo, ROOT_TREE_NODE};
use bmbp_util::{uuid, uuid_upper, TreeBuilder};
use std::os::unix::process::parent_id;

pub struct MenuService;
#[allow(dead_code)]
impl MenuService {
    pub(crate) async fn find_tree(params: &MenuQueryParam) -> BmbpResp<Vec<BmbpMenuVo>> {
        let vo_vec = MenuDao::find_list(params).await?;
        let tree_vo = TreeBuilder::build(vo_vec);
        Ok(tree_vo)
    }

    pub(crate) async fn find_page(
        params: &PageReqVo<MenuQueryParam>,
    ) -> BmbpResp<PageInner<BmbpMenuVo>> {
        let vo_vec = MenuDao::find_page(params).await?;
        Ok(vo_vec)
    }

    pub(crate) async fn find_list(params: &MenuQueryParam) -> BmbpResp<Vec<BmbpMenuVo>> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn find_one(params: &MenuQueryParam) -> BmbpResp<Option<BmbpMenuVo>> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn delete(params: &MenuQueryParam) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn insert(po: &mut BmbpMenuVo) -> BmbpResp<usize> {
        append_create_vo::<BmbpMenuVo>(po);
        Self::append_create_menu_vo(po);
        tracing::debug!("插入菜单:{:#?}", po);
        MenuDao::insert(po).await
    }
    pub(crate) fn append_create_menu_vo(po: &mut BmbpMenuVo) {
        po.set_menu_id(uuid_upper().to_string());
        // 获取上级菜单路径
        if po.get_parent_menu_id().is_empty() {
            po.set_parent_menu_id(ROOT_TREE_NODE.to_string());
            po.set_menu_path(format!("/{}", po.get_menu_path()));
        } else {
            //TODO 获取上级菜单
        };
    }

    pub(crate) async fn update(params: &MenuQueryParam, po: &mut BmbpMenuVo) -> BmbpResp<usize> {
        append_update_vo::<BmbpMenuVo>(po);
        Self::append_update_menu_vo(po);
        MenuDao::update(params, po).await
    }
    pub(crate) fn append_update_menu_vo(po: &mut BmbpMenuVo) {
        po.set_menu_id(uuid_upper().to_string());
        // 获取上级菜单路径
        if po.get_parent_menu_id().is_empty() {
            po.set_parent_menu_id(ROOT_TREE_NODE.to_string());
            po.set_menu_path(format!("/{}", po.get_menu_path()));
        } else {
            //TODO 获取上级菜单
        };
    }
    pub(crate) async fn save(po: &mut BmbpMenuVo) -> BmbpResp<usize> {
        if po.get_r_id().is_empty() {
            Self::insert(po).await
        } else {
            let params = MenuQueryParam::default();
            Self::update(&params, po).await
        }
    }
}
