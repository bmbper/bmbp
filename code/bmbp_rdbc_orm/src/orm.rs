use std::sync::Arc;
use bmbp_rdbc_sql::{Delete, Insert, Query, Update};
use crate::ds::RdbcDataSource;
use crate::pool::{RdbcConn, RdbcConnPool};
use crate::err::RdbcResult;
use crate::val::{RdbcModel, RdbcPage, RdbcRow};


pub struct RdbcOrmInner {
    datasource: Arc<RdbcDataSource>,
    pool: RdbcConnPool,
}

impl RdbcOrmInner {
    pub async fn new(data_source: RdbcDataSource) -> RdbcResult<Self> {
        let arc_ds = Arc::new(data_source);
        let arc_pool = RdbcConnPool::new(arc_ds.clone());
        arc_pool.init().await?;
        Ok(RdbcOrmInner {
            datasource: arc_ds.clone(),
            pool: arc_pool,
        })
    }
}

impl RdbcOrmInner {
    pub async fn get_conn(&self) -> RdbcResult<RdbcConn> {
        self.pool.get_conn().await
    }
    pub async fn valid(&self) -> bool {
        self.pool.valid().await
    }
    pub async fn select_page_by_query<T>(&self, page: &mut RdbcPage<T>, query: &Query) -> RdbcResult<RdbcPage<T>> where T: From<RdbcRow> {
        Ok(RdbcPage::new())
    }
    pub async fn select_list_by_query<T>(&self, query: &Query) -> RdbcResult<Option<Vec<T>>> where T: From<RdbcRow> {
        Ok(None)
    }
    pub async fn select_one_by_query<T>(&self, query: &Query) -> RdbcResult<Option<T>> where T: From<RdbcRow> {
        Ok(None)
    }
    pub async fn execute_insert(&self, insert: &Insert) -> RdbcResult<u64> {
        Ok(0)
    }
    pub async fn execute_update(&self, insert: &Update) -> RdbcResult<u64> {
        Ok(0)
    }
    pub async fn execute_delete(&self, insert: &Delete) -> RdbcResult<u64> {
        Ok(0)
    }
    pub async fn delete_by_id<T>(&self,id:String) -> RdbcResult<u64> where T:RdbcModel { Ok(0) }
}
