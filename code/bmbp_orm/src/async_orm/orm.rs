use std::sync::Arc;

use serde_json::{Map, Value};

use bmbp_types::{BmbpResp, PageInner};

use crate::{BmbpDataSource, BmbpOrmSQL};

use super::pool::BmbpConnectionPool;

#[allow(dead_code)]
pub struct Orm {
    data_source: Arc<BmbpDataSource>,
    pool: Arc<BmbpConnectionPool>,
}

impl Orm {
    pub async fn new(data_source: Arc<BmbpDataSource>) -> BmbpResp<Arc<Orm>> {
        let arc_pool = BmbpConnectionPool::new(data_source.clone()).await?;
        let orm = Orm {
            data_source: data_source.clone(),
            pool: arc_pool.clone(),
        };
        Ok(Arc::new(orm))
    }

    pub fn data_source(&self) -> Arc<BmbpDataSource> {
        self.data_source.clone()
    }
}

impl Orm {
    pub async fn find_page(
        &self,
        orm_sql: BmbpOrmSQL,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageInner<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let page = conn.find_page(orm_sql, page_no, page_size).await;
        conn.release().await;
        page
    }

    pub async fn find_list(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<Vec<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let vec_vo = conn.find_list(orm_sql).await;
        conn.release().await;
        vec_vo
    }

    pub async fn find_one(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<Option<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let vo = conn.find_one(orm_sql).await;
        conn.release().await;
        vo
    }

    pub async fn delete(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.delete(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn insert(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.insert(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn update(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.update(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn execute(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn execute_ddl(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute_ddl(orm_sql).await;
        conn.release().await;
        row_count
    }
    pub async fn execute_dml(&self, orm_sql: BmbpOrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute_dml(orm_sql).await;
        conn.release().await;
        row_count
    }
}

/// JSON操修作,根据JSON进行参数、反回值处理
impl Orm {}
