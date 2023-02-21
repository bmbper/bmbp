use crate::menu::dao::MenuDao;
use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};
use crate::util::{append_create_vo, append_update_vo};
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BmbpError, BmbpPageReqVo, BmbpResp, PageInner, ROOT_TREE_NODE};
use bmbp_util::{uuid, uuid_upper, TreeBuilder};
use std::os::unix::process::parent_id;

pub struct MenuService;
#[allow(dead_code)]
impl MenuService {
    pub(crate) async fn find_tree(params: &MenuQueryParam) -> BmbpResp<Vec<BmbpMenuVo>> {
        let vo_vec = MenuDao::find_list(params).await?;
        let mut tree_vo = TreeBuilder::build(vo_vec);
        tree_vo
            .sort_by_key(|key| format!("{}-{}", key.get_menu_order().clone(), key.get_menu_path()));
        Ok(tree_vo)
    }

    pub(crate) async fn find_page(
        params: &BmbpPageReqVo<MenuQueryParam>,
    ) -> BmbpResp<PageInner<BmbpMenuVo>> {
        let vo_vec = MenuDao::find_page(params).await?;
        Ok(vo_vec)
    }

    pub(crate) async fn find_list(params: &MenuQueryParam) -> BmbpResp<Vec<BmbpMenuVo>> {
        let vo_vec = MenuDao::find_list(params).await?;
        Ok(vo_vec)
    }

    pub(crate) async fn find_one(params: &MenuQueryParam) -> BmbpResp<Option<BmbpMenuVo>> {
        let vo = MenuDao::find_one(params).await?;
        Ok(vo)
    }

    pub(crate) async fn delete(params: &MenuQueryParam) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn insert(po: &mut BmbpMenuVo) -> BmbpResp<usize> {
        append_create_vo::<BmbpMenuVo>(po);
        Self::append_create_menu_vo(po).await?;
        MenuDao::insert(po).await
    }

    pub(crate) async fn append_create_menu_vo(po: &mut BmbpMenuVo) -> BmbpResp<()> {
        po.set_menu_id(uuid_upper().to_string());

        // 设置为根节点
        let mut set_root_node = |tpo: &mut BmbpMenuVo| {
            tpo.set_parent_menu_id(ROOT_TREE_NODE.to_string());
            tpo.set_menu_path(format!("/{}/", tpo.get_menu_title()));
        };

        // 获取上级菜单路径
        if po.get_parent_menu_id().is_empty() {
            set_root_node(po);
        } else {
            let parent_menu_path = Self::find_parent_menu_path(po.get_parent_menu_id()).await?;
            if let Some(menu_path) = parent_menu_path {
                po.set_menu_path(format!("{}{}/", menu_path, po.get_menu_title()));
            } else {
                set_root_node(po);
            }
        }
        Ok(())
    }

    pub(crate) async fn update(params: &MenuQueryParam, po: &mut BmbpMenuVo) -> BmbpResp<usize> {
        append_update_vo::<BmbpMenuVo>(po);
        Self::append_update_menu_vo(po).await;
        MenuDao::update(params, po).await
    }
    pub(crate) async fn append_update_menu_vo(po: &mut BmbpMenuVo) {}

    async fn find_parent_menu_path(parent_menu_id: &String) -> BmbpResp<Option<String>> {
        let mut menu_query_param = MenuQueryParam::default();
        menu_query_param.set_menu_id(parent_menu_id.to_string());
        let menu_vo = Self::find_one(&menu_query_param).await?;
        if let Some(vo) = menu_vo {
            Ok(Some(vo.get_menu_path().to_string()))
        } else {
            Ok(None)
        }
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
