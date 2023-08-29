use async_trait::async_trait;
use bmbp_app_common::{BmbpHashMap, BmbpResp, PageVo};
#[async_trait]
pub trait CurdDaoTrait {
    async fn find_page(
        script_sql: &String,
        script_params: &BmbpHashMap,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_query_page(
                script_sql,
                script_params,
                page_no.clone(),
                page_size.clone(),
            )
            .await
    }

    async fn find_list(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_query_list(script_sql, script_params)
            .await
    }

    async fn find_info(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_query_one(script_sql, script_params)
            .await
    }

    async fn insert(script_sql: &String, script_params: &mut BmbpHashMap) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_insert(script_sql, script_params)
            .await
    }

    async fn update(script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_update(script, params)
            .await
    }

    async fn delete(script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_delete(script, params)
            .await
    }
}
