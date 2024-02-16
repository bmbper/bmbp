use std::sync::Arc;
use async_trait::async_trait;
use crate::conn::RdbcDbConn;
use crate::RdbcDataSource;

pub struct PgDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl PgDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> Self {
        PgDbClient {
            data_source: data_source.clone()
        }
    }
}
#[async_trait]
impl RdbcDbConn for PgDbClient {
   async fn is_valid(&self) -> bool {
        return true;
    }
}