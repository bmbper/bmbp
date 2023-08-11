use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::PageVo;

use super::model::BmbpRbacOrgan;

pub struct OrganDao();
#[allow(dead_code)]
impl OrganDao {
    pub(crate) async fn find_organ_tree(
        sql_scirpt: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        bmbp_orm_ins::BmbpORM
            .await
            .generate_script_query_list::<BmbpRbacOrgan>(sql_scirpt, params)
            .await
    }

    pub(crate) async fn find_organ_page(
        script_sql: &String,
        script_params: &BmbpHashMap,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageVo<BmbpRbacOrgan>> {
        bmbp_orm_ins::BmbpORM
            .await
            .generate_script_query_page(
                script_sql,
                script_params,
                page_no.clone(),
                page_size.clone(),
            )
            .await
    }

    pub(crate) async fn find_organ_list(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        bmbp_orm_ins::BmbpORM
            .await
            .generate_script_query_list::<BmbpRbacOrgan>(script_sql, script_params)
            .await
    }

    pub(crate) async fn find_organ_info(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        bmbp_orm_ins::BmbpORM
            .await
            .generate_script_query_one::<BmbpRbacOrgan>(script_sql, script_params)
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
