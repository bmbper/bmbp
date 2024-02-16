use std::sync::Arc;
use async_trait::async_trait;
use crate::client::pg::PgDbClient;
use crate::client::sqlite::SqliteDbClient;
use crate::conn::RdbcDbConn;
use crate::RdbcDataSource;

pub struct MysqlDbClient{
    data_source: Arc<RdbcDataSource>
}
impl MysqlDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> Self {
        MysqlDbClient {
            data_source: data_source.clone()
        }
    }
}
#[async_trait]
impl RdbcDbConn for MysqlDbClient {
    async fn is_valid(&self) -> bool {
        return true;
    }

}