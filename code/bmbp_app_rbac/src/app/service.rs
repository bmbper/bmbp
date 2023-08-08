use bmbp_app_common::{BmbpHashMap, BmbpResp, BmbpValue, PageVo};

use crate::app::dao::RbacAppDao;

use super::{
    model::{AppQueryParams, BmbpRbacApp},
    script::RbacAppScript,
};

pub struct RbacAppService;
impl RbacAppService {
    pub(crate) async fn find_page(params: &AppQueryParams) -> BmbpResp<PageVo<BmbpRbacApp>> {
        let query_script = RbacAppScript::query_script();
        let query_params = BmbpHashMap::new();
        if let Some(app) = params.get_params() {}
        let page_vo = RbacAppDao::find_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }
}
