use std::sync::Arc;
use async_trait::async_trait;
use crate::conn::RdbcDbConn;
use crate::RdbcDataSource;

pub struct SqliteDbClient {
    data_source: Arc<RdbcDataSource>,
}
impl SqliteDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> Self {
        SqliteDbClient {
            data_source: data_source.clone()
        }
    }

}
#[async_trait]
impl RdbcDbConn for SqliteDbClient {
    async  fn is_valid(&self) -> bool {
        return true;
    }
}