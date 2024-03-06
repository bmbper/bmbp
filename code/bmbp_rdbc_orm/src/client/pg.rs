use std::fmt::Debug;
use std::sync::{Arc};
use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_postgres::{Client, connect, Error, NoTls, Row};
use tokio_postgres::types::{ToSql};
use bmbp_rdbc_macro::RdbcOrmRow;
use bmbp_rdbc_sql::{Delete, Insert, Query, RdbcSQL, RdbcSQLParser, RdbcValue, Update};
use crate::err::{RdbcError, RdbcErrorType, RdbcResult};
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct PgDbClient {
    data_source: Arc<RdbcDataSource>,
    client: RwLock<Client>,
}


impl RdbcSQLParser for PgDbClient {
    fn to_query(&self, query: &Query) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_insert(&self, query: &Insert) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_update(&self, query: &Update) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_delete(&self, query: &Delete) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}


impl PgDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        let url = Self::build_url(data_source.clone())?;
        match connect(url.as_str(), NoTls).await {
            Ok((client, conn)) => {
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                Ok(PgDbClient {
                    data_source: data_source.clone(),
                    client: RwLock::new(client),
                })
            }
            Err(e) => {
                Err(RdbcError::new(RdbcErrorType::ConnectError, &e.to_string()))
            }
        }
    }
    fn build_url(ds: Arc<RdbcDataSource>) -> RdbcResult<String> {
        Ok(format!(
            "postgresql://{}:{}@{}:{}/{}?connect_timeout={}",
            ds.user(),
            ds.password(),
            ds.host(),
            ds.port(),
            ds.database(),
            ds.max_wait_time().unwrap_or(5_000),
        ))
    }
}

impl PgDbClient {
    fn to_pg_sql(value: &RdbcValue) -> Option<&(dyn ToSql + Sync)> {
        match value {
            RdbcValue::Int(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::BigInt(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::Float(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::BigFloat(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::String(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::DateTime(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::Bool(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::Null => {
                { Some(&"" as &(dyn ToSql + Sync)) }
            }
        }
    }
    async fn execute(&self, sql: &str, params: &[RdbcValue]) -> RdbcResult<u64> {
        let pg_prams = params.iter().filter_map(|v| Self::to_pg_sql(v)).collect::<Vec<_>>();
        match self.client.read().await.execute(sql, pg_prams.as_slice()).await {
            Ok(row_count) => {
                Ok(row_count)
            }
            Err(e) => {
                Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string()))
            }
        }
    }
}

#[async_trait]
impl RdbcConnInner for PgDbClient {
    async fn valid(&self) -> bool {
        let test_url = "select 1";
        self.client.read().await.execute(test_url, &[]).await.is_ok()
    }
    async fn select_list_by_query(&self, query: &Query) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        let (pg_sql, page_prams) = self.to_query(query);
        self.select_list_by_sql(pg_sql.as_str(), pg_sql.as_slice()).await
    }
    async fn select_one_by_query(&self, query: &Query) -> RdbcResult<Option<RdbcOrmRow>> {
        let (sql, params) = query.to_sql_params();
        let pg_prams = params.iter().filter_map(|v| Self::to_pg_sql(v)).collect::<Vec<_>>();
        match self.client.read().await.query_opt(sql.as_str(), pg_prams.as_slice()).await {
            Ok(row_op) => {
                if let Some(row) = row_op {
                    Ok(Some(RdbcOrmRow::from(row)))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string()))
            }
        }
    }
    async fn select_list_by_sql(&self, query: &str, params: &[RdbcValue]) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        let pg_prams = params.iter().filter_map(|v| Self::to_pg_sql(v)).collect::<Vec<_>>();
        match self.client.read().await.query(query, pg_prams.as_slice()).await {
            Ok(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let orm_row = RdbcOrmRow::from(row);
                    list.push(orm_row);
                }
                Ok(Some(list))
            }
            Err(e) => {
                Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string()))
            }
        }
    }

    async fn execute_insert(&self, insert: &Insert) -> RdbcResult<u64> {
        let (sql, params) = insert.to_sql_with_params();
        self.execute(sql.as_str(), params.as_slice()).await
    }

    async fn execute_update(&self, update: &Update) -> RdbcResult<u64> {
        let (sql, params) = update.to_sql_with_params();
        self.execute(sql.as_str(), params.as_slice()).await
    }

    async fn execute_delete(&self, delete: &Delete) -> RdbcResult<u64> {
        let (sql, params) = delete.to_sql_with_params();
        self.execute(sql.as_str(), params.as_slice()).await
    }
}
