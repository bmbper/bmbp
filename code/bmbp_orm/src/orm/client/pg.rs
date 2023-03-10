use std::{
    borrow::BorrowMut,
    sync::{Arc, Weak},
};

use async_trait::async_trait;
use serde_json::{Map, Value};
use tokio::sync::{Mutex, RwLock};
use tokio_postgres::{connect, types::ToSql, Client, Error, NoTls, Row};
use tracing::debug;

use bmbp_types::{BmbpError, BmbpResp, PageInner};
use bmbp_util::uuid;

use crate::{
    orm::{conn::BmbpConn, pool::BmbpConnectionPool},
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
                    id: uuid(),
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
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageInner<Map<String, Value>>> {
        let pg_params = to_pg_prams(params);
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> &(dyn ToSql + Sync) { x.as_ref() })
            .collect::<Vec<&(dyn ToSql + Sync)>>();
        let mut page_inner = PageInner::new();
        page_inner.set_page_no(page_no.clone());
        page_inner.set_page_size(page_size.clone());

        let count_sql = format!("SELECT COUNT(1) AS count FROM ({}) AS t1", &sql);
        tracing::info!("???????????????????????????SQL:{}", &count_sql);
        let count_params = pg_params_ref.as_slice();
        let count_rs = self
            .client
            .lock()
            .await
            .query_one(count_sql.as_str(), count_params)
            .await;
        let err = match count_rs {
            Ok(row) => {
                let total_count: i64 = row.get("count");
                page_inner.set_total(total_count as usize);
                None
            }
            Err(err) => Some(Err(BmbpError::orm(err.to_string()))),
        };
        if err.is_some() {
            return err.unwrap();
        }

        let limit_size = { *page_size };
        let offset_no = {
            if page_no < &1 {
                0
            } else {
                page_size * (page_no - 1)
            }
        };

        let page_sql = format!(" {} LIMIT {} OFFSET {}", &sql, limit_size, offset_no);
        let list_data = self.find_list(page_sql, params).await?;
        page_inner.set_data(list_data);
        Ok(page_inner)
    }

    async fn find_list(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Vec<Map<String, Value>>> {
        tracing::info!("????????????SQL:{}", &sql);
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

    async fn find_one(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Option<Map<String, Value>>> {
        tracing::info!("??????????????????SQL:{}", &sql);
        let pg_params = to_pg_prams(params);
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> &(dyn ToSql + Sync) { x.as_ref() })
            .collect::<Vec<&(dyn ToSql + Sync)>>();
        let rows_rs = self
            .client
            .lock()
            .await
            .query_opt(sql.as_str(), pg_params_ref.as_slice())
            .await;
        match rows_rs {
            Ok(row_opt) => {
                if let Some(row) = row_opt {
                    Ok(Some(to_json_value(&row)))
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                let err_msg = err.to_string();
                tracing::error!("{}", err_msg);
                Err(BmbpError::orm(err_msg))
            }
        }
    }

    async fn insert(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("????????????SQL:{}", &sql);
        self.execute(sql, params).await
    }
    #[allow(unused)]
    async fn update(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("????????????SQL:{}", &sql);
        self.execute(sql, params).await
    }
    #[allow(unused)]
    async fn delete(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("????????????SQL:{},params:{:#?}", &sql, params);
        self.execute(sql, params).await
    }
    #[allow(unused)]
    async fn execute(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        let pg_params = to_pg_prams(params);
        let pg_params_ref = pg_params
            .iter()
            .map(|x| -> &(dyn ToSql + Sync) { x.as_ref() })
            .collect::<Vec<&(dyn ToSql + Sync)>>();
        debug!("pg_sql:{}", sql);
        debug!("pa_params:{:#?}", pg_params_ref);
        let execute_rs = self
            .client
            .lock()
            .await
            .execute(sql.as_str(), pg_params_ref.as_slice())
            .await;
        match execute_rs {
            Ok(row_count) => Ok(row_count as usize),
            Err(err) => Err(BmbpError::orm(err.to_string())),
        }
    }

    #[allow(unused)]
    async fn execute_ddl(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("??????DDL SQL:{}", &sql);
        self.execute(sql, params).await
    }
    #[allow(unused)]
    async fn execute_dml(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        tracing::info!("??????DML SQL:{}", &sql);
        self.execute(sql, params).await
    }

    async fn batch_execute(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize> {
        Ok(0)
    }

    async fn batch_execute_ddl(&mut self, ddl_vec: &[(String, &[Value])]) -> BmbpResp<usize> {
        Ok(0)
    }

    async fn batch_execute_dml(&mut self, dml_vec: &[(String, &[Value])]) -> BmbpResp<usize> {
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
            Value::String(v) => pg_params.push(Box::new(v.to_string())),
            Value::Array(_) => {}
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
        let mut props_value = Value::Null;
        match col_type {
            "varchar" => {
                let v_rs: Result<String, Error> = row.try_get(col_name);
                match v_rs {
                    Ok(v) => {
                        props_value = Value::String(v);
                    }
                    Err(_) => {
                        props_value = Value::Null;
                    }
                }
            }
            "int2" | "int4" | "init8" => {
                let v_rs: Result<i32, Error> = row.try_get(col_name);
                match v_rs {
                    Ok(v) => {
                        props_value = Value::from(v);
                    }
                    Err(_) => {
                        props_value = Value::Null;
                    }
                }
            }
            _ => {
                tracing::warn!("{}?????????????????????{}", col_name, col_type);
            }
        }
        map.insert(col_name.to_string(), props_value);
    }
    map
}
