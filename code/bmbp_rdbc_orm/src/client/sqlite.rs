use std::sync::Arc;
use async_trait::async_trait;
use crate::err::RdbcResult;
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct SqliteDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl SqliteDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        Ok(SqliteDbClient {
            data_source: data_source.clone()
        })
    }
}

#[async_trait]
impl RdbcConnInner for SqliteDbClient {
    async fn valid(&self) -> bool {
        return true;
    }
}