use std::collections::HashMap;
use std::{
    borrow::BorrowMut,
    sync::{Arc, Weak},
};

use async_trait::async_trait;
use serde_json::{Map, Value};
use tokio::sync::{Mutex, RwLock};
use tokio_postgres::types::Type;
use tokio_postgres::{connect, types::ToSql, Client, NoTls, Row};

use bmbp_types::{BmbpError, BmbpResp, PageInner};
use bmbp_util::id;
use bmbp_util::string::BmbpStringUtil;

use crate::{
    async_orm::{conn::BmbpConn, pool::BmbpConnectionPool},
    BmbpDataSource,
};

#[warn(dead_code)]
pub struct BmbpPgConnect {
    id: String,
    data_source: Arc<BmbpDataSource>,
    pool: Weak<BmbpConnectionPool>,
    client: Mutex<Client>,
}

impl BmbpPgConnect {
    pub async fn new(
        ds: Arc<BmbpDataSource>,
    ) -> BmbpResp<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>> {
        let client_rs = connect(BmbpPgConnect::database_url(ds.clone()).as_str(), NoTls).await;
        match client_rs {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                let conn = BmbpPgConnect {
                    id: id::uuid(),
                    data_source: ds.clone(),
                    pool: Weak::new(),
                    client: Mutex::new(client),
                };
                let box_conn: Box<dyn BmbpConn + Send + Sync + 'static> = Box::new(conn);
                Ok(RwLock::new(box_conn))
            }
            Err(e) => Err(BmbpError::orm(format!("{:#?}", e))),
        }
    }
    fn database_url(ds: Arc<BmbpDataSource>) -> String {
        let ds = ds.clone();
        format!(
            "postgresql://{}:{}@{}:{}/{}?connect_timeout=10",
            ds.user(),
            ds.password(),
            ds.host(),
            ds.port(),
            ds.database()
        )
    }
}

#[async_trait]
impl BmbpConn for BmbpPgConnect {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn set_pool(&mut self, pool: Arc<BmbpConnectionPool>) {
        *(self.pool.borrow_mut()) = Arc::downgrade(&pool.clone());
    }
    fn pool(&self) -> Arc<BmbpConnectionPool> {
        self.pool.upgrade().unwrap().clone()
    }

    fn set_data_source(&mut self, data_source: Arc<BmbpDataSource>) {
        self.data_source = data_source.clone();
    }

    fn data_source(&self) -> Arc<BmbpDataSource> {
        self.data_source.clone()
    }

    async fn find_page(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<PageInner<Map<String, Value>>> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(PageInner::default())
    }

    async fn find_list(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Vec<Map<String, Value>>> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        let pg_params = to_pg_prams(params);
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> &(dyn ToSql + Sync) { x.as_ref() })
            .collect::<Vec<&(dyn ToSql + Sync)>>();
        let rows_rs = self
            .client
            .lock()
            .await
            .query(sql.as_str(), pg_params_ref.as_slice())
            .await;
        match rows_rs {
            Ok(rows) => {
                let mut value_rows = vec![];
                for row in rows {
                    let row_value = to_json_value(&row);
                    value_rows.push(row_value)
                }
                Ok(value_rows)
            }
            Err(err) => Err(BmbpError::orm(err.to_string())),
        }
    }

    async fn find_one(&mut self, sql: String, params: &[Value]) -> BmbpResp<Map<String, Value>> {
        tracing::info!("执行Postgresql\nSQL:  {}\n参数:  {:#?}", sql, params);
        Ok(Map::new())
    }

    async fn insert(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }
    async fn update(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }
    async fn delete(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }
    async fn execute(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }

    async fn execute_ddl(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }

    async fn execute_dml(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("执行Postgresql\n SQL:{} \n 参数:{:#?}", sql, params);
        Ok(0)
    }
}

fn to_pg_prams(params: &[Value]) -> Vec<Box<(dyn ToSql + Send + Sync + 'static)>> {
    let mut pg_params: Vec<Box<(dyn ToSql + Send + Sync + 'static)>> = vec![];
    for item in params {
        match item {
            Value::Null => {}
            Value::Bool(v) => pg_params.push(Box::new(v.clone())),
            Value::Number(v) => {
                if v.is_f64() {
                    pg_params.push(Box::new(v.as_f64().unwrap()))
                } else if v.is_i64() {
                    pg_params.push(Box::new(v.as_i64().unwrap()))
                } else if v.is_u64() {
                    pg_params.push(Box::new(v.as_u64().unwrap() as i64))
                }
            }
            Value::String(v) => pg_params.push(Box::new(v.clone())),
            _ => {}
        }
    }
    pg_params
}

fn to_json_value(row: &Row) -> Map<String, Value> {
    let mut map = Map::new();
    let columns = row.columns();
    for column in columns {
        let col_name = column.name();
        let col_type = column.type_().name();
        let props = BmbpStringUtil::snake_to_camel(col_name.clone().to_string());
        let mut props_value = Value::Null;
        match col_type {
            "varchar" => {
                let v: String = row.get(col_name);
                props_value = Value::String(v);
            }
            "int2" | "int4" | "init8" => {
                let v: u32 = row.get(col_name);
                props_value = Value::from(v);
            }
            _ => {}
        }
        map.insert(props, props_value);
    }
    map
}
