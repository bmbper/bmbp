use bmbp_app_common::{BmbpHashMap, BmbpResp, BmbpValue, PageVo};

use crate::app::dao::RbacAppDao;

use super::{
    model::{AppQueryParams, BmbpRbacApp},
    script::RbacAppScript,
};

pub struct RbacAppService;
impl RbacAppService {
    pub(crate) async fn find_page(params: &AppQueryParams) -> BmbpResp<PageVo<BmbpHashMap>> {
        let query_script = RbacAppScript::query_script();
        let query_params = BmbpHashMap::new();
        if let Some(_) = params.get_params() {}
        let page_vo = RbacAppDao::find_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }

    pub(crate) async fn find_list(_params: &BmbpRbacApp) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let query_script = RbacAppScript::query_script();
        let query_params = BmbpHashMap::new();
        let page_vo = RbacAppDao::find_list(&query_script.to_script(), &query_params).await?;
        Ok(page_vo)
    }

    pub(crate) async fn find_info(record_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut params = BmbpHashMap::new();
        params.insert("recordId".to_string(), BmbpValue::from(record_id));
        let mut query_script = RbacAppScript::query_script();
        query_script.filter("record_id = #{recordId}");
        let app_info = RbacAppDao::find_info(&query_script.to_script(), &params).await?;
        Ok(app_info)
    }
}
