use crate::dict::model::BmbpSettingDictOrmModel;
use bmbp_app_common::{BmbpError, BmbpResp, PageVo};
use bmbp_rdbc_orm::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcORM, UpdateWrapper};

pub struct BmbpRbacDictDao {}

impl BmbpRbacDictDao {
    pub async fn select_page_by_query(
        page_no: &usize,
        page_size: &usize,
        query: &QueryWrapper,
    ) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
        match RdbcORM
            .await
            .select_page_by_query::<BmbpSettingDictOrmModel>(
                page_no.clone(),
                page_size.clone(),
                &query,
            )
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
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        match RdbcORM
            .await
            .select_list_by_query::<BmbpSettingDictOrmModel>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn select_one_by_query(
        query: &QueryWrapper,
    ) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        match RdbcORM
            .await
            .select_one_by_query::<BmbpSettingDictOrmModel>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_insert(insert: &InsertWrapper) -> BmbpResp<usize> {
        match RdbcORM.await.execute_insert(insert).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_update(update: &UpdateWrapper) -> BmbpResp<usize> {
        match RdbcORM.await.execute_update(update).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_delete(delete_dict: &DeleteWrapper) -> BmbpResp<usize> {
        match RdbcORM.await.execute_delete(delete_dict).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
}
