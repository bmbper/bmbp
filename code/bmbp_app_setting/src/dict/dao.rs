use bmbp_app_common::{BmbpError, BmbpResp, PageVo};
use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcORM, RdbcPage, Update};
use crate::dict::model::BmbpSettingDictOrmModel;

pub struct BmbpRbacDictDao {}

impl BmbpRbacDictDao {
    pub async fn select_page_by_query(page_no: &usize, page_size: &usize, query: &Query) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
        let mut page: RdbcPage<BmbpSettingDictOrmModel> = RdbcPage::new();
        page.set_page_num(page_no.clone()).set_page_size(page_size().clone());
        match RdbcORM.await.select_page_by_query::<BmbpSettingDictOrmModel>(&mut page, &query).await {
            Ok(page_) => {
                let rbac_page = PageVo::new_page(page_.page_num().clone(),
                                                 page_.page_size().clone(), page_.total().clone(), page.data_take());
                Ok(rbac_page)
            }
            Err(e) => Err(BmbpError::service(e.get_msg().as_str()))
        }
    }
    pub async fn select_list_by_query(query: &Query) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        match RdbcORM.await.select_list_by_query::<BmbpSettingDictOrmModel>(&query).await {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
        }
    }
    pub async fn select_one_by_query(query: &Query) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        match RdbcORM.await.select_one_by_query::<BmbpSettingDictOrmModel>(&query).await {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
        }
    }
    pub async fn execute_insert(insert: &Insert) -> BmbpResp<usize> {
        match RdbcORM.await.execute_insert(insert).await {
            Ok(data) => {
                Ok(data as usize)
            }
            Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
        }
    }
    pub async fn execute_update(update: &Update) -> BmbpResp<usize> {
        match RdbcORM.await.execute_update(update).await {
            Ok(data) => {
                Ok(data as usize)
            }
            Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
        }
    }
    pub async fn execute_delete(delete_dict: &Delete) -> BmbpResp<usize> {
        match RdbcORM.await.execute_delete(delete_dict).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
        }
    }
}

