use std::sync::Arc;
use crate::conn::RdbcDbConn;
use crate::RdbcDataSource;

pub struct SqliteDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl RdbcDbConn for SqliteDbClient {
    async fn new(data_source: Arc<RdbcDataSource>) -> Self {
        SqliteDbClient {
            data_source: data_source.clone()
        }
    }

    fn is_valid(&self) -> bool {
        return true;
    }
}