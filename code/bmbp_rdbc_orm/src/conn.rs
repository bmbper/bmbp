use std::sync::{Arc};
use crate::datasource::RdbcDataSource;

pub trait RdbcDbConn {
    async fn new(data_source:Arc<RdbcDataSource>)->Self;
    async fn is_valid(&self) -> bool;
}
