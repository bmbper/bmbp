mod pg;
mod mysql;
mod sqlite;

use std::sync::Arc;
use crate::client::mysql::MysqlDbClient;
use crate::client::pg::PgDbClient;
use crate::client::sqlite::SqliteDbClient;
use crate::conn::RdbcDbConn;
use crate::RdbcDataBaseDriver::*;
use crate::RdbcDataSource;

pub async fn build_conn(ds: Arc<RdbcDataSource>) -> Box<dyn RdbcDbConn + Send + Sync + 'static> {
    match ds.driver() {
        Mysql => {
            return build_mysql_conn(ds.clone()).await;
        }
       Postgresql => {
            return build_postgres_conn(ds.clone()).await;
        }
        Sqlite => {
            return build_sqlite_conn(ds.clone()).await;
        }
        _ => {
            panic!("not support driver")
        }
    }
}

async fn build_sqlite_conn(datasource: Arc<RdbcDataSource>) -> Box<dyn RdbcDbConn + Send + Sync + 'static> {
    Box::new(SqliteDbClient::new(datasource.clone()).await)
}

async fn build_postgres_conn(datasource: Arc<RdbcDataSource>) -> Box<dyn RdbcDbConn + Send + Sync + 'static> {
    Box::new(PgDbClient::new(datasource.clone()).await)
}

async fn build_mysql_conn(datasource: Arc<RdbcDataSource>) -> Box<dyn RdbcDbConn + Send + Sync + 'static> {
    Box::new(MysqlDbClient::new(datasource.clone()).await)
}

