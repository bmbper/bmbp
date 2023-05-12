use std::{
    borrow::BorrowMut,
    sync::{Arc, Weak},
};

use async_trait::async_trait;
use serde_json::{Map, Value};
use tokio::sync::RwLock;

use bmbp_types::{BmbpResp, PageRespVo};
use bmbp_util::uuid;

use crate::{
    orm::{conn::BmbpConn, pool::BmbpConnectionPool},
    BmbpDataSource,
};

pub struct BmbpSqliteConnect {
    id: String,
    data_source: Arc<BmbpDataSource>,
    pool: Weak<BmbpConnectionPool>,
}

impl BmbpSqliteConnect {
    pub async fn new(
        ds: Arc<BmbpDataSource>,
    ) -> BmbpResp<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>> {
        let con = BmbpSqliteConnect {
            id: uuid(),
            data_source: ds.clone(),
            pool: Weak::new(),
        };
        Ok(RwLock::new(Box::new(con)))
    }
}

#[allow(dead_code)]
#[async_trait]
impl BmbpConn for BmbpSqliteConnect {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn set_pool(&mut self, pool: Arc<BmbpConnectionPool>) {
        *(self.pool.borrow_mut()) = Arc::downgrade(&pool.clone());
    }
    fn pool(&self) -> Arc<BmbpConnectionPool> {
        self.pool.upgrade().unwrap().clone()
    }

    fn set_data_source(&mut self, data_source: Arc<BmbpDataSource>) {
        self.data_source = data_source.clone();
    }

    fn data_source(&self) -> Arc<BmbpDataSource> {
        self.data_source.clone()
    }
    #[allow(unused)]
    async fn find_page(
        &mut self,
        sql: String,
        params: &[Value],
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageRespVo<Map<String, Value>>> {
        Ok(PageRespVo::default())
    }
    #[allow(unused)]
    async fn find_list(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Vec<Map<String, Value>>> {
        Ok(vec![])
    }
    #[allow(unused)]
    async fn find_one(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Option<Map<String, Value>>> {
        Ok(None)
    }
    #[allow(unused)]
    async fn insert(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(1)
    }
    #[allow(unused)]
    async fn update(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn delete(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn execute(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn execute_ddl(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn execute_dml(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn batch_execute(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn batch_execute_ddl(&mut self, ddl_vec: &[(String, &[Value])]) -> BmbpResp<usize> {
        Ok(0)
    }
    #[allow(unused)]
    async fn batch_execute_dml(&mut self, dml_vec: &[(String, &[Value])]) -> BmbpResp<usize> {
        Ok(0)
    }
}
