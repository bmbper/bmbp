use std::sync::Arc;
use crate::conn::RdbcDbConn;
use crate::RdbcDataSource;

pub struct PgDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl RdbcDbConn for PgDbClient {
    async fn new(data_source: Arc<RdbcDataSource>) -> Self {
        PgDbClient {
            data_source: data_source.clone()
        }
    }

    fn is_valid(&self) -> bool {
        return true;
    }
}