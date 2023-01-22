use crate::menu::dao::MenuDao;
use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};
use bmbp_types::{BmbpError, BmbpResp, PageInner, PageReqVo};
use bmbp_util::TreeBuilder;

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

    pub(crate) async fn insert<BmbpMenuVo>(po: &BmbpMenuVo) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn update(params: &MenuQueryParam, po: &BmbpMenuVo) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) async fn save(po: &BmbpMenuVo) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
}

pub struct MenuValidator;
impl MenuValidator {}
