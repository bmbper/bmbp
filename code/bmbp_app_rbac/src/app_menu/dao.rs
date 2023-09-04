use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageVo;

pub struct MenuDao();
#[allow(dead_code)]
impl MenuDao {
    pub(crate) async fn find_menu_page(
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

    pub(crate) async fn find_menu_list(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_query_list(script_sql, script_params)
            .await
    }

    pub(crate) async fn find_menu_info(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_query_one(script_sql, script_params)
            .await
    }

    pub(crate) async fn insert(
        script_sql: &String,
        script_params: &mut BmbpHashMap,
    ) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_insert(script_sql, script_params)
            .await
    }

    pub(crate) async fn update(script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_update(script, params)
            .await
    }

    pub(crate) async fn delete(script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        bmbp_orm_ins::BmbpORM
            .await
            .script_delete(script, params)
            .await
    }
}
