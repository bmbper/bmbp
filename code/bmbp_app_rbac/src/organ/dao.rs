use bmbp_app_common::{BmbpError, BmbpResp, PageVo};
use bmbp_app_orm::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcOrmIns, UpdateWrapper};

use crate::organ::model::BmbpRbacOrganTree;

pub struct BmbpRbacOrganDao;
impl BmbpRbacOrganDao {
    pub async fn select_page_by_query(
        page_no: &usize,
        page_size: &usize,
        query: &QueryWrapper,
    ) -> BmbpResp<PageVo<BmbpRbacOrganTree>> {
        match RdbcOrmIns
            .await
            .select_page_by_query::<BmbpRbacOrganTree>(page_no.clone(), page_size.clone(), &query)
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
    pub async fn select_list_by_query(
        query: &QueryWrapper,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        match RdbcOrmIns
            .await
            .select_list_by_query::<BmbpRbacOrganTree>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn select_one_by_query(query: &QueryWrapper) -> BmbpResp<Option<BmbpRbacOrganTree>> {
        match RdbcOrmIns
            .await
            .select_one_by_query::<BmbpRbacOrganTree>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_insert(insert: &InsertWrapper) -> BmbpResp<usize> {
        match RdbcOrmIns.await.execute_insert(insert).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_update(update: &UpdateWrapper) -> BmbpResp<usize> {
        match RdbcOrmIns.await.execute_update(update).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_delete(delete_dict: &DeleteWrapper) -> BmbpResp<usize> {
        match RdbcOrmIns.await.execute_delete(delete_dict).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
}
