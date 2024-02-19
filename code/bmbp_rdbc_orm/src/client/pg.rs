use std::sync::{Arc};
use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_postgres::{Client, connect, NoTls};
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

#[async_trait]
impl RdbcConnInner for PgDbClient {
    async fn valid(&self) -> bool {
        let test_url = "select 1";
        self.client.read().await.execute(test_url, &[]).await.is_ok()
    }
}