use std::{
    collections::HashMap,
    sync::{Arc, Weak},
    time::Duration,
    vec,
};

use serde_json::{Map, Value};
use time::OffsetDateTime;
use tokio::sync::{Mutex, RwLock};
use tracing::debug;

use bmbp_types::{BmbpError, BmbpResp, PageInner};

use crate::{BmbpDataSource, BmbpOrmSQL};

use super::{client::build_conn, conn::BmbpConn};

pub struct BmbpConnectionPool {
    data_source: Arc<BmbpDataSource>,
    conn_map: RwLock<HashMap<String, Arc<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>>>>,
    idel_conn_id: Mutex<Vec<String>>,
    active_conn_id: Mutex<Vec<String>>,
}

impl BmbpConnectionPool {
    pub async fn new(data_source: Arc<BmbpDataSource>) -> BmbpResp<Arc<BmbpConnectionPool>> {
        let conn_map = RwLock::new(HashMap::new());
        let pool = BmbpConnectionPool {
            data_source: data_source,
            conn_map,
            idel_conn_id: Mutex::new(vec![]),
            active_conn_id: Mutex::new(vec![]),
        };
        let arc_pool = Arc::new(pool);
        arc_pool.init_connection(arc_pool.clone()).await?;
        Ok(arc_pool)
    }

    pub async fn init_connection(&self, arc_pool: Arc<BmbpConnectionPool>) -> BmbpResp<()> {
        tracing::info!("初始化数据连接池....");
        let mut mp = self.conn_map.write().await;
        let init_pool_size = self.data_source.pool_config().init_size().clone();
        for _ in 0..init_pool_size.clone() {
            let conn = build_conn(self.data_source.clone()).await?;
            let arc_conn = Arc::new(conn);
            arc_conn.write().await.set_pool(arc_pool.clone());
            mp.insert(arc_conn.read().await.id().clone(), arc_conn.clone());
            self.idel_conn_id
                .lock()
                .await
                .push(arc_conn.clone().read().await.id().clone());
        }
        tracing::info!("连接池初始化完成,连接数{}", init_pool_size);
        Ok(())
    }
}

impl BmbpConnectionPool {
    pub async fn get_conn(&self) -> BmbpResp<ConnInner> {
        let mut conn_id_op = self.idel_conn_id.lock().await.pop();
        let cur_now = OffsetDateTime::now_utc().second();
        while conn_id_op.is_none() {
            conn_id_op = self.idel_conn_id.lock().await.pop();
            let end_now = OffsetDateTime::now_utc().second();
            if end_now - cur_now >= 5 {
                return Err(BmbpError::orm("获取数据库连接池超时:5秒".to_string()));
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        let conn_id = conn_id_op.unwrap();
        let arc_conn = self
            .conn_map
            .read()
            .await
            .get(conn_id.as_str())
            .unwrap()
            .clone();
        self.active_conn_id.lock().await.push(conn_id.clone());
        Ok(ConnInner::new(arc_conn.clone()).await)
    }

    pub async fn release_conn(&self, conn_id: String) {
        self.idel_conn_id.lock().await.push(conn_id.clone());
        self.active_conn_id
            .lock()
            .await
            .retain(|x| x.clone() == conn_id);
    }
}

pub struct ConnInner {
    id: String,
    pool: Weak<BmbpConnectionPool>,
    inner: Arc<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>>,
}

impl ConnInner {
    pub async fn new(inner: Arc<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>>) -> ConnInner {
        let conn_id = inner.clone().read().await.id();
        let coon_pool = inner.clone().read().await.pool();
        let weak_pool = Arc::downgrade(&coon_pool.clone());
        ConnInner {
            inner,
            id: conn_id,
            pool: weak_pool,
        }
    }
    pub fn inner(&self) -> &Arc<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>> {
        &self.inner
    }

    pub async fn release(&self) {
        self.pool
            .upgrade()
            .unwrap()
            .clone()
            .release_conn(self.id.clone())
            .await;
    }
}

impl ConnInner {
    pub async fn find_page(
        &self,
        mut orm_sql: BmbpOrmSQL,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageInner<Map<String, Value>>> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let page = self
            .inner()
            .write()
            .await
            .find_page(sql, params, page_no, page_size)
            .await?;
        Ok(page)
    }

    pub async fn find_list(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<Vec<Map<String, Value>>> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let vec_vo = self.inner().write().await.find_list(sql, params).await?;
        Ok(vec_vo)
    }

    pub async fn find_one(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<Option<Map<String, Value>>> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let vo = self.inner().write().await.find_one(sql, params).await?;
        Ok(vo)
    }
    pub async fn insert(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        debug!("SQL:{}", sql);
        debug!("PARAMS:{}", sql);
        let row_count = self.inner().write().await.insert(sql, params).await?;
        Ok(row_count)
    }
    pub async fn update(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let row_count = self.inner().write().await.update(sql, params).await?;
        Ok(row_count)
    }
    pub async fn delete(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let row_count = self.inner().write().await.delete(sql, params).await?;
        Ok(row_count)
    }

    pub async fn execute(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let row_count = self.inner().write().await.execute(sql, params).await?;
        Ok(row_count)
    }
    pub async fn execute_dml(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let row_count = self.inner().write().await.execute_dml(sql, params).await?;
        Ok(row_count)
    }
    pub async fn execute_ddl(&self, mut orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let (sql, params) = orm_sql.get_raw_orm()?;
        let row_count = self.inner().write().await.execute_ddl(sql, params).await?;
        Ok(row_count)
    }
}
