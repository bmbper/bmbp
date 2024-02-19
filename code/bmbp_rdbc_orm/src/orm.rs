use std::sync::Arc;
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
    pub async fn valid(self) -> bool {
        self.pool.valid().await
    }
}
