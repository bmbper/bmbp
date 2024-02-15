use crate::conn::{RdbcDbConn};
use crate::datasource::RdbcDataSource;
use crate::pool::RdbcConnPool;
use std::sync::Arc;
pub struct RdbcOrm {
    datasource: Arc<RdbcDataSource>,
    pool: Arc<RdbcConnPool>,
}
impl RdbcOrm {
    pub async fn new(data_source: RdbcDataSource) -> Self {
        let arc_ds = Arc::new(data_source);
        let mut arc_pool = RdbcConnPool::new(arc_ds.clone());
        arc_pool.init().await;
        RdbcOrm {
            datasource: arc_ds.clone(),
            pool: arc_pool,
        }
    }
}
impl RdbcOrm {
    pub async fn get_conn(&self) -> Box<dyn RdbcDbConn + Send + Sync + 'static> {
        self.pool.get_conn().await
    }
}
