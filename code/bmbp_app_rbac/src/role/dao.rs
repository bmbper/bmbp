use bmbp_app_common::{BmbpError, BmbpResp, PageVo};
use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcORM, Update};

use crate::role::model::BmbpRbacRoleModel;

pub struct BmbpRbacRoleDao;
impl BmbpRbacRoleDao { 
    pub async fn select_page_by_query(
    page_no: &usize,
    page_size: &usize,
    query: &Query,
) -> BmbpResp<PageVo<BmbpRbacRoleModel>> {
    match RdbcORM
        .await
        .select_page_by_query::<BmbpRbacRoleModel>(page_no.clone(), page_size.clone(), &query)
        .await
    {
        Ok(mut page) => {
            let mut page_vo = PageVo::new();
            page_vo.set_page_no(page.page_num().clone());
            page_vo.set_page_size(page.page_size().clone());
            page_vo.set_op_data(page.data_take());
            page_vo.set_row_total(page.total().clone());
            Ok(page_vo)
        }
        Err(e) => Err(BmbpError::service(e.get_msg().as_str())),
    }
}
    pub async fn select_list_by_query(query: &Query) -> BmbpResp<Option<Vec<BmbpRbacRoleModel>>> {
        match RdbcORM
            .await
            .select_list_by_query::<BmbpRbacRoleModel>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn select_one_by_query(query: &Query) -> BmbpResp<Option<BmbpRbacRoleModel>> {
        match RdbcORM
            .await
            .select_one_by_query::<BmbpRbacRoleModel>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_insert(insert: &Insert) -> BmbpResp<usize> {
        match RdbcORM.await.execute_insert(insert).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_update(update: &Update) -> BmbpResp<usize> {
        match RdbcORM.await.execute_update(update).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_delete(delete_dict: &Delete) -> BmbpResp<usize> {
        match RdbcORM.await.execute_delete(delete_dict).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }}
