use std::cell::RefCell;
use crate::conn::{RdbcDbConn};
use crate::datasource::RdbcDataSource;
use std::sync::{Arc, Weak};
use std::sync::{RwLock};
use crate::client;

pub struct RdbcConnPool {
    data_source: Arc<RdbcDataSource>,
    conn_size: RwLock<usize>,
    conn_map: RwLock<Vec<Box<dyn RdbcDbConn + Send + Sync + 'static>>>,
    _re: RefCell<Weak<RdbcConnPool>>,
}

impl RdbcConnPool {
    pub fn new(data_source: Arc<RdbcDataSource>) -> Arc<RdbcConnPool> {
        let pool = RdbcConnPool {
            data_source,
            conn_size: RwLock::new(0),
            conn_map: RwLock::new(vec![]),
            _re: RefCell::new(Weak::new()),
        };
        let arc_pool = Arc::new(pool);
        *arc_pool._re.borrow_mut() = Arc::downgrade(&arc_pool) ;
        arc_pool
    }

    pub async fn init(&self) {
        let ds = self.data_source.clone();
        let init_conn_size = ds.init_conn_size().unwrap_or(1);
        *self.conn_size.write().unwrap() = init_conn_size.clone();
        for _ in 0..init_conn_size {
            let conn = client::build_conn(ds.clone()).await;
            self.conn_map.write().unwrap().push(conn);
        }
    }

    pub async fn build_conn_grown(&self) {
        let ds = self.data_source.clone();
        let max_conn_size = ds.max_conn_size().unwrap_or(10);
        let mut conn_size = self.conn_size.read().unwrap().clone();
        let grow_size = 5;
        if conn_size + grow_size < max_conn_size {
            for _ in 0..5 {
                let conn = client::build_conn(ds.clone()).await;
                self.conn_map.write().unwrap().push(conn);
            }
        }
        *self.conn_size.write().unwrap() = conn_size + grow_size;
    }
}

impl RdbcConnPool {
    pub async fn get_conn(&self) -> RdbcConn {
        if self.conn_map.read().unwrap().is_empty() {
            self.build_conn_grown().await;
        }
        let conn = self.conn_map.write().unwrap().pop().unwrap();
        RdbcConn {
            name: "x".to_string(),
            datasource: self.data_source.clone(),
            pool: self._re.borrow().clone(),
            conn: Some(conn),
        }
    }
    pub fn push_conn(&self, conn: Box<dyn RdbcDbConn + Send + Sync + 'static>) {
        self.conn_map.write().unwrap().push(conn);
    }
}

pub struct RdbcConn {
    pub(crate) name: String,
    pub(crate) datasource: Arc<RdbcDataSource>,
    pub(crate) pool: Weak<RdbcConnPool>,
    pub(crate) conn: Option<Box<dyn RdbcDbConn + Send + Sync + 'static>>,
}

impl RdbcConn {
    pub async fn is_valid(&mut self) -> bool {
        self.conn.as_mut().unwrap().is_valid().await
    }
}

impl Drop for RdbcConn {
    fn drop(&mut self) {
        let con = self.conn.take().unwrap();
        self.pool.upgrade().unwrap().push_conn(con)
    }
}
