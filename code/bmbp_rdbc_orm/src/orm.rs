use std::fmt::Debug;
use std::sync::Arc;
use serde::Serialize;
use bmbp_rdbc_macro::{RdbcModel, RdbcOrmRow, RdbcPage};
use bmbp_rdbc_sql::{Delete, Insert, Query, Update};
use crate::ds::RdbcDataSource;
use crate::pool::{RdbcConn, RdbcConnPool};
use crate::err::RdbcResult;


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
    pub async fn select_page_by_query<'a, T>(&self, page: &'a mut RdbcPage<T>, query: &Query) -> RdbcResult<&'a mut RdbcPage<T>> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
        Ok(page)
    }
    pub async fn select_list_by_query<T>(&self, query: &Query) -> RdbcResult<Option<Vec<T>>> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
        let row_op = self.pool.get_conn().await?.select_list_by_query(query).await?;
        match row_op {
            Some(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let t = T::from(row);
                    list.push(t);
                }
                Ok(Some(list))
            }
            None => Ok(None)
        }
    }
    pub async fn select_one_by_query<T>(&self, query: &Query) -> RdbcResult<Option<T>> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
        let row_op = self.pool.get_conn().await?.select_one_by_query(query).await?;
        match row_op {
            Some(row) => {
                Ok(Some(T::from(row)))
            }
            None => Ok(None)
        }
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
    pub async fn delete_by_id<T>(&self, id: String) -> RdbcResult<u64> where T: RdbcModel { Ok(0) }
}
