use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use bmbp_app_common::{BmbpError, BmbpHashMap, BmbpResp, BmbpValue, PageVo};

use super::pool::BmbpConnectionPool;
use crate::script::ScriptUtil;
use crate::{BmbpDataSource, BmbpOrmSQL};

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
    ) -> BmbpResp<PageVo<Map<String, Value>>> {
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
#[allow(unused)]
impl Orm {
    pub async fn dynamic_query_page(
        &self,
        orm: &mut BmbpOrmSQL,
    ) -> BmbpResp<PageVo<Option<BmbpHashMap>>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_query_list(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn dynamic_query_one(&self, orm: &mut BmbpOrmSQL) -> BmbpResp<Option<BmbpHashMap>> {
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
#[allow(unused)]
impl Orm {
    pub async fn raw_query_page(
        &self,
        sql: &String,
        page_no: usize,
        page_size: usize,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let conn = self.pool.get_conn().await?;
        let model_page = conn.raw_find_page(sql, &[], page_no, page_size).await;
        conn.release().await;
        model_page
    }
    pub async fn raw_query_page_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
        page_no: usize,
        page_size: usize,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let conn = self.pool.get_conn().await?;
        let model_page = conn.raw_find_page(sql, params, page_no, page_size).await;
        conn.release().await;
        model_page
    }

    pub async fn raw_query_list(&self, sql: &String) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let conn = self.pool.get_conn().await?;
        let model_vec = conn.raw_find_list(sql, &[]).await;
        conn.release().await;
        model_vec
    }

    pub async fn raw_query_list_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let conn = self.pool.get_conn().await?;
        let model_vec = conn.raw_find_list(sql, params).await;
        conn.release().await;
        model_vec
    }

    pub async fn raw_query_one(&self, sql: &String) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn raw_query_one_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<Option<BmbpHashMap>> {
        let conn = self.pool.get_conn().await?;
        let model_vec = conn.raw_find_one(sql, params).await;
        conn.release().await;
        model_vec
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

    pub async fn raw_insert(&self, sql: &String) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;

        let model_vec = conn.raw_insert(sql, &[]).await;
        conn.release().await;
        model_vec
    }

    pub async fn raw_insert_with_params(
        &self,
        sql: &String,
        params: &[BmbpValue],
    ) -> BmbpResp<usize> {
        let conn = self.pool.get_conn().await?;
        let model_vec = conn.raw_insert(sql, params).await;
        conn.release().await;
        model_vec
    }

    pub async fn raw_insert_batch_with_params(
        &self,
        sql: &String,
        params: &[&[BmbpValue]],
    ) -> BmbpResp<PageVo<usize>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn batch_raw_insert(&self, sql: &[String]) -> BmbpResp<PageVo<usize>> {
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
        let conn = self.pool.get_conn().await?;
        let row_count = conn.raw_update(sql, params).await;
        conn.release().await;
        row_count
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
#[allow(unused)]
impl Orm {
    pub async fn script_query_page(
        &self,
        script: &String,
        params: &BmbpHashMap,
        page_no: usize,
        page_size: usize,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let (sql, params) = ScriptUtil::parse_from_map(script, params.clone());
        self.raw_query_page_with_params(&sql, params.as_slice(), page_no, page_size)
            .await
    }
    pub async fn script_query_list(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let (sql, params) = ScriptUtil::parse_from_map(script, params.clone());
        self.raw_query_list_with_params(&sql, params.as_slice())
            .await
    }
    pub async fn script_query_one(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        let (sql, params) = ScriptUtil::parse_from_map(script, params.clone());
        self.raw_query_one_with_params(&sql, params.as_slice())
            .await
    }
    pub async fn script_query_value(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }

    pub async fn script_insert(&self, script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        println!("===>script sql:{:#?}", script);
        let (sql, params) = ScriptUtil::parse_from_map(script, params.clone());
        self.raw_insert_with_params(&sql, params.as_slice()).await
    }
    pub async fn script_insert_batch(
        &self,
        script: &String,
        params: &[&BmbpHashMap],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_insert(
        &self,
        script: &[String],
        params: &[&BmbpHashMap],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_insert_slice(
        &self,
        batch_script_params: &[&(String, &BmbpHashMap)],
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_update(&self, script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        let (sql, params) = ScriptUtil::parse_from_map(script, params.clone());
        self.raw_update_with_params(&sql, params.as_slice()).await
    }
    pub async fn script_insert_update(
        &self,
        script: &String,
        params: &[&BmbpHashMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_update(
        &self,
        script: &[String],
        params: &[&BmbpHashMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_update_slice(
        &self,
        batch_script_params: &[&(String, &BmbpHashMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_delete(&self, script: &String, params: &BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_insert_delete(
        &self,
        script: &String,
        params: &[&BmbpHashMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_delete(
        &self,
        script: &[String],
        params: &[&BmbpHashMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_delete_slice(
        &self,
        batch_script_params: &[&(String, &BmbpHashMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn script_execute(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<BmbpValue>> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_execute(
        &self,
        script: &[String],
        params: &[&BmbpHashMap],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
    pub async fn batch_script_execute_slice(
        &self,
        batch_script_params: &[&(String, &BmbpHashMap)],
    ) -> BmbpResp<usize> {
        Err(BmbpError::orm("方法未实现".to_string()))
    }
}

/// ORM 泛型处理调用方法
#[allow(unused)]
impl Orm {
    /// 查询分页并转为实际的类型
    pub async fn generate_script_query_page<T>(
        &self,
        script: &String,
        params: &BmbpHashMap,
        page_no: usize,
        page_size: usize,
    ) -> BmbpResp<PageVo<T>>
    where
        T: Default + Clone + Serialize + for<'a> Deserialize<'a> + Send + Sync,
    {
        let rs = self
            .script_query_page(script, params, page_no, page_size)
            .await?;
        let gen_rs = rs.get_data();
        let rs_1 = match gen_rs {
            None => None,
            Some(v) => {
                let js = serde_json::to_string_pretty(&v).unwrap().clone();
                tracing::info!("{}", js);
                let rs = serde_json::from_str(&js);
                rs.unwrap()
            }
        };
        let mut new_page = PageVo::new();
        new_page.set_page_no(rs.get_page_no().unwrap().clone());
        new_page.set_page_size(rs.get_page_size().unwrap().clone());
        new_page.set_row_total(rs.get_row_total().unwrap().clone());
        if rs_1.is_some() {
            new_page.set_data(rs_1.unwrap());
        }
        Ok(new_page)
    }
    /// 查询分页 并转为实际类型
    pub async fn generate_script_query_list<T>(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<Vec<T>>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        let rs = self.script_query_list(script, params).await?;
        let rsp = match rs {
            None => None,
            Some(v) => {
                let js = serde_json::to_string(&v).unwrap();
                let rs = serde_json::from_str(&js);
                rs.unwrap()
            }
        };
        Ok(rsp)
    }
    /// 查询详情，并转为实际类弄
    pub async fn generate_script_query_one<T>(
        &self,
        script: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Option<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        let rs = self.script_query_one(script, params).await?;
        let rsp = match rs {
            None => None,
            Some(v) => {
                let js = serde_json::to_string(&v).unwrap();
                println!("bmbp->js:{}", js);
                let rs = serde_json::from_str(&js);
                rs.unwrap()
            }
        };
        Ok(rsp)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{BmbpDataSource, Orm};
    use bmbp_app_common::BmbpVec;
    use tracing::info;

    #[tokio::test]
    async fn test_orm_sql() {
        let datasource = Arc::new(BmbpDataSource::new());
        let orm = Orm::new(datasource).await.unwrap();

        let raw_insert_batch_with_params_script = "".to_string();
        let mut batch_params = vec![];
        let params = BmbpVec::new();
        batch_params.push(params.as_slice());
        let rs = orm
            .raw_insert_batch_with_params(
                &raw_insert_batch_with_params_script,
                batch_params.as_slice(),
            )
            .await;

        match rs {
            Ok(v) => {
                info!("{:#?}", v);
            }
            Err(e) => {
                info!("{:#?}", e);
            }
        }
        let mut batch_sql = vec![];
        let one_sql = "".to_string();
        let one_params = BmbpVec::new();
        let one_sql_params = &(one_sql, one_params.as_slice());
        batch_sql.push(one_sql_params);
        _ = orm.batch_raw_delete_with_slice(batch_sql.as_slice()).await;
    }
}
