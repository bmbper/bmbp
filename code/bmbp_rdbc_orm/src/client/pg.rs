use std::fmt::Debug;
use std::sync::{Arc};
use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_postgres::{Client, connect, NoTls};
use tokio_postgres::types::{ToSql};
use bmbp_rdbc_macro::RdbcOrmRow;
use bmbp_rdbc_sql::{Query,RdbcValue};
use crate::err::{RdbcError, RdbcErrorType, RdbcResult};
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct PgDbClient {
    data_source: Arc<RdbcDataSource>,
    client: RwLock<Client>,
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
            RdbcValue::BigFloat(i) =>{ Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::String(i) =>{ Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::DateTime(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::Bool(i) => { Some(i as &(dyn ToSql + Sync)) }
            RdbcValue::Null => {
                { Some(&"" as &(dyn ToSql + Sync)) }
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
        let (sql,params) = query.to_sql_params();
        self.select_list(sql.as_str(), params.as_slice()).await
    }
    async fn select_list(&self, query: &str, params: &[RdbcValue]) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        let pg_prams = params.iter().filter_map(|v|Self::to_pg_sql(v)).collect::<Vec<_>>();
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
}
