use super::model::BmbpRbacApp;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::{BmbpHashMap, PageVo};

pub struct RbacAppDao;
impl RbacAppDao {
    pub async fn find_page(
        script_sql: &String,
        script_params: &BmbpHashMap,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageVo<BmbpRbacApp>> {
        bmbp_orm_ins::BmbpORM
            .await
            .generate_script_query_page::<BmbpRbacApp>(
                script_sql,
                script_params,
                page_no.clone(),
                page_size.clone(),
            )
            .await
    }
}
