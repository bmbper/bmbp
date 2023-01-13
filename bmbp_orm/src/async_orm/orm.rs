use std::sync::Arc;

use serde_json::{Map, Value};

use bmbp_types::{BmbpResp, PageInner};

use crate::{BmbpDataSource, OrmSQL};

use super::pool::BmbpConnectionPool;

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

/// 原生接口,根据BmbpOrmValue进行操作
impl Orm {
    pub async fn find_page(
        &self,
        mut orm_sql: OrmSQL,
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageInner<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let page = conn.find_page(orm_sql, page_no, page_size).await;
        conn.release().await;
        page
    }

    pub async fn find_list(&self, mut orm_sql: OrmSQL) -> BmbpResp<Vec<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let vec_vo = conn.find_list(orm_sql).await;
        conn.release().await;
        vec_vo
    }

    pub async fn find_one(&self, mut orm_sql: OrmSQL) -> BmbpResp<Option<Map<String, Value>>> {
        let conn = self.pool.get_conn().await?;
        let vo = conn.find_one(orm_sql).await;
        conn.release().await;
        vo
    }

    pub async fn delete(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.delete(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn insert(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.insert(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn update(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.update(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn execute(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute(orm_sql).await;
        conn.release().await;
        row_count
    }

    pub async fn execute_ddl(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute_ddl(orm_sql).await;
        conn.release().await;
        row_count
    }
    pub async fn execute_dml(&self, mut orm_sql: OrmSQL) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.execute_dml(orm_sql).await;
        conn.release().await;
        row_count
    }
}

/// JSON操修作,根据JSON进行参数、反回值处理
impl Orm {}
