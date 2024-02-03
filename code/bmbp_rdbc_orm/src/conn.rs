use std::sync::{Arc, RwLock, Weak};
use crate::datasource::RdbcDataSource;
use crate::pool::RdbcConnPool;

pub trait RdbcDbConn {}

pub struct RdbcConn {
    data_source: Arc<RdbcDataSource>,
    pool: Weak<RdbcConnPool>,
    inner: Option<Box<dyn RdbcDbConn + Send + Sync + 'static>>,
}

impl RdbcConn {
    pub fn new(pool: Arc<RdbcConnPool>, ds: Arc<RdbcDataSource>, conn: Option<Box<dyn RdbcDbConn + Send + Sync>>) -> RdbcConn {
        RdbcConn {
            data_source: ds,
            pool: pool.downgrade(),
            inner: conn,
        }
    }
}

impl Drop for RdbcConn {
    fn drop(&mut self) {
        self.pool.upgrade().unwrap().push_conn(self.inner.take());
    }
}