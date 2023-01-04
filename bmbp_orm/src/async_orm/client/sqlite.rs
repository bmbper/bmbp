use std::{
    borrow::BorrowMut,
    sync::{Arc, Weak},
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use bmbp_types::BmbpResp;
use bmbp_util::uuid;

use crate::{
    async_orm::{conn::BmbpConn, pool::BmbpConnectionPool},
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
}
