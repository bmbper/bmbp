use std::sync::Arc;
use async_trait::async_trait;
use bmbp_rdbc_macro::RdbcOrmRow;
use bmbp_rdbc_sql::{Query, RdbcValue};
use crate::err::RdbcResult;
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct MysqlDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl MysqlDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        Ok(MysqlDbClient {
            data_source: data_source.clone()
        })
    }
}

#[async_trait]
impl RdbcConnInner for MysqlDbClient {
    async fn valid(&self) -> bool {
        return true;
    }
    async fn select_list_by_query(&self, query: &Query) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }
    async fn select_one_by_query(&self, query: &Query) -> RdbcResult<Option<RdbcOrmRow>> {
        Ok(None)
    }
    async fn select_list(&self, query: &str, params: &[RdbcValue]) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }
}