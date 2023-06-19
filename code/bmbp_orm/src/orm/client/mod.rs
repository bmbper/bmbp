use std::sync::Arc;

use tokio::sync::RwLock;

use bmbp_app_common::BmbpResp;
pub use mysql::BmbpMysqlConnect;
pub use pg::BmbpPgConnect;
pub use sqlite::BmbpSqliteConnect;

use crate::BmbpDataSource;

use super::conn::BmbpConn;

mod mysql;
mod pg;
mod sqlite;

pub async fn build_conn(
    ds: Arc<BmbpDataSource>,
) -> BmbpResp<RwLock<Box<dyn BmbpConn + Send + Sync + 'static>>> {
    match ds.clone().driver().as_str() {
        "postgres" => BmbpPgConnect::new(ds).await,
        "mysql" => BmbpMysqlConnect::new(ds).await,
        "sqlite" => BmbpSqliteConnect::new(ds).await,
        _ => BmbpPgConnect::new(ds).await,
    }
}
