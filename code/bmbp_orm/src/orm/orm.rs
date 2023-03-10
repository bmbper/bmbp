use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use bmbp_types::{BmbpError, BmbpResp, PageInner};

use crate::orm::{BmbpMap, BmbpValue, BmbpVec};
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

/// ORM 方法 逐步废弃
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
    pub async fn batch_execute(&self, orm_sql: &mut [BmbpOrmSQL]) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let row_count = conn.batch_execute(orm_sql).await;
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

/// ORM 动态SQL方法
impl Orm {
    pub async fn dynamic_query_page(
        &self,
        orm: &mut BmbpOrmSQL,
    ) -> BmbpResp<PageInner<Option<BmbpMap>>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_query_list(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<Option<BmbpMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_query_one(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<Option<BmbpMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_query_value(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<BmbpValue> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_insert(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_update(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_delete(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_execute(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_execute_ddl(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_execute_dml(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_dynamic_insert(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_dynamic_update(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_dynamic_delete(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_dynamic_execute(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_dynamic_delete_dml(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_dynamic_execute_ddl(&self, orm: &mut [BmbpOrmSQL]) -> BmbpResp<Vec<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
}

// ORM 原生SQL调用方法
impl Orm {
    pub async fn raw_query_page(&self, sql: &String) -> BmbpResp<PageInner<Option<BmbpVec>>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn raw_query_page_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<PageInner<Option<BmbpVec>>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_list(&self, sql: &String) -> BmbpResp<Option<BmbpVec>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_list_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<Option<BmbpVec>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_one(&self, sql: &String) -> BmbpResp<Option<BmbpMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_one_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<Option<BmbpVec>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_value(&self, sql: &String) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_value_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_insert(&self, sql: &String) -> BmbpResp<PageInner<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_insert_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_insert_batch_with_params(
        &self,
        sql: &String,
        params: &[&[BmbpValue]],
    ) -> BmbpResp<PageInner<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_insert(&self, sql: &[String]) -> BmbpResp<PageInner<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_insert_with_params(
        &self,
        sql: &[String],
        params: &[&[BmbpValue]],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_insert_with_slice(
        &self,
        batch_sql_params: &[(&String, &[BmbpValue])],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_update(&self, sql: &String) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_update_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_update_batch_with_params(
        &self,
        sql: &String,
        params: &[&[BmbpValue]],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_update(&self, sql: &[String]) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_update_with_params(
        &self,
        sql: &[String],
        params: &[BmbpValue],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_update_with_slice(
        &self,
        batch_sql_prams: &[&(String, &[BmbpValue])],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn raw_delete(&self, sql: &String) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn raw_delete_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_delete_batch_with_params(
        &self,
        sql: &String,
        params: &[&[BmbpValue]],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_raw_delete(&self, sql: &[String]) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_raw_delete_with_prams(
        &self,
        sql: &[String],
        params: &[&[BmbpValue]],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_raw_delete_with_slice(
        &self,
        batch_sql_params: &[&(String, &[BmbpValue])],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn raw_execute_ddl(&self, sql: &String) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_raw_execute_ddl(&self, sql: &[String]) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
}

// ORM的脚本调用方法
impl Orm {
    pub async fn script_query_page(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<PageInner<Option<BmbpMap>>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_query_list(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<Option<BmbpMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_query_one(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<Option<BmbpMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_query_value(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn script_insert(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_insert_batch(
        &self,
        script: &String,
        params: &[&BmbpMap],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_insert(
        &self,
        script: &[String],
        params: &[&BmbpMap],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_insert_slice(
        &self,
        batch_script_params: &[&(String, &BmbpMap)],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_update(&self, script: &String, params: &BmbpMap) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_insert_update(
        &self,
        script: &String,
        params: &[&BmbpMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_update(
        &self,
        script: &[String],
        params: &[&BmbpMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_update_slice(
        &self,
        batch_script_params: &[&(String, &BmbpMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_delete(&self, script: &String, params: &BmbpMap) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_insert_delete(
        &self,
        script: &String,
        params: &[&BmbpMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_delete(
        &self,
        script: &[String],
        params: &[&BmbpMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_delete_slice(
        &self,
        batch_script_params: &[&(String, &BmbpMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_execute(
        &self,
        script: &String,
        params: &BmbpMap,
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_execute(
        &self,
        script: &[String],
        params: &[&BmbpMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_execute_slice(
        &self,
        batch_script_params: &[&(String, &BmbpMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
}

/// ORM 泛型处理调用方法
impl Orm {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::orm::{BmbpMap, BmbpVec};
    use crate::{BmbpDataSource, Orm};

    #[tokio::test]
    async fn test_orm_sql() {
        let datasource = Arc::new(BmbpDataSource::new());
        let orm = Orm::new(datasource).await.unwrap();

        let raw_insert_batch_with_params_script = "".to_string();
        let mut batch_params = vec![];
        let params = BmbpVec::new();
        batch_params.push(params.as_slice());
        orm.raw_insert_batch_with_params(
            &raw_insert_batch_with_params_script,
            batch_params.as_slice(),
        );

        let mut batch_sql = vec![];
        let one_sql = "".to_string();
        let one_params = BmbpVec::new();
        let one_sql_params = &(one_sql, one_params.as_slice());
        batch_sql.push(one_sql_params);
        orm.batch_raw_delete_with_slice(batch_sql.as_slice());
    }
}
