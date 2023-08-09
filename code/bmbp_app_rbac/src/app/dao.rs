use bmbp_app_common::BmbpResp;
use bmbp_app_common::{BmbpHashMap, PageVo};

pub struct RbacAppDao;
impl RbacAppDao {
    pub async fn find_page(
        script_sql: &String,
        script_params: &BmbpHashMap,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_query_page(
                script_sql,
                script_params,
                page_no.clone(),
                page_size.clone(),
            )
            .await?;
        Ok(rs)
    }

    pub async fn find_list(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_query_list(script_sql, script_params)
            .await?;
        Ok(rs)
    }

    pub(crate) async fn find_info(to_script: &String, params: &BmbpHashMap) -> Option<BmbpHashMap> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_query_one(script_sql, script_params)
            .await?;
        Ok(rs)
    }
}
