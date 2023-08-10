use bmbp_app_common::{BmbpError, BmbpResp};
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

    pub(crate) async fn find_info(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_query_one(script_sql, script_params)
            .await?;
        Ok(rs)
    }
    pub(crate) async fn update_status(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<usize> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_update(script_sql, script_params)
            .await?;
        Ok(rs)
    }

    pub(crate) async fn delete_app(
        script_sql: &String,
        script_params: &BmbpHashMap,
    ) -> BmbpResp<usize> {
        let rs = bmbp_orm_ins::BmbpORM
            .await
            .script_delete(script_sql, script_params)
            .await?;
        Ok(rs)
    }
    pub(crate) async fn insert_app(_params: &mut BmbpHashMap) -> BmbpResp<&mut BmbpHashMap> {
        Err(BmbpError::dao("保存未实现"))
    }
    pub(crate) async fn update_app(_params: &mut BmbpHashMap) -> BmbpResp<&mut BmbpHashMap> {
        Err(BmbpError::dao("更新未实现"))
    }
}
