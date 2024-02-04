use crate::conn::{RdbcConn, RdbcDbConn};
use crate::datasource::RdbcDataSource;
use std::sync::Arc;
use std::sync::{RwLock, Weak};

pub struct RdbcConnPool {
    data_source: Arc<RdbcDataSource>,
    conn_map: RwLock<Vec<Box<dyn RdbcDbConn + Send + Sync + 'static>>>,
    self_pool: Weak<RdbcConnPool>,
}

impl RdbcConnPool {
    pub async fn new(data_source: Arc<RdbcDataSource>) -> Arc<RdbcConnPool> {
        let pool = RdbcConnPool {
            data_source,
            conn_map: RwLock::new(vec![]),
            self_pool: Default::default(),
        };
        let arc_pool = Arc::new(pool);
        arc_pool
    }
}

impl RdbcConnPool {
    pub async fn get_conn(&self) -> RdbcConn {
        let pool = self.self_pool.upgrade().unwrap().clone();
        RdbcConn::new(
            pool,
            self.data_source.clone(),
            self.conn_map.write().unwrap().pop(),
        )
    }
    pub fn push_conn(&self, conn: Box<dyn RdbcDbConn + Send + Sync + 'static>) {
        self.conn_map.write().unwrap().push(conn);
    }
}
