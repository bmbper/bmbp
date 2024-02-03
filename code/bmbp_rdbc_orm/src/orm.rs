use std::sync::{Arc};
use crate::conn::RdbcConn;
use crate::datasource::RdbcDataSource;
use crate::pool::RdbcConnPool;
pub struct RdbcOrm {
    datasource: Arc<RdbcDataSource>,
    pool: Arc<RdbcConnPool>,
}
impl RdbcOrm {
    pub fn new(data_source: RdbcDataSource) -> Self {
        let arc_ds = Arc::new(data_source);
        RdbcOrm {
            datasource: arc_ds.clone(),
            pool: RdbcConnPool::new(arc_ds.clone()),
        }
    }
}
impl RdbcOrm {
    pub async fn get_conn(&self) -> RdbcConn {
        self.pool.get_conn()
    }
}
