use crate::err::RdbcResult;
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;
use async_trait::async_trait;
use bmbp_rdbc_macro::RdbcOrmRow;
use bmbp_rdbc_sql::{Delete, Insert, Query, RdbcValue, Update};
use std::sync::Arc;

pub struct SqliteDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl SqliteDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        Ok(SqliteDbClient {
            data_source: data_source.clone(),
        })
    }
}

#[async_trait]
impl RdbcConnInner for SqliteDbClient {
    async fn valid(&self) -> bool {
        return true;
    }

    async fn select_list_by_query(&self, query: &Query) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }
    async fn select_one_by_query(&self, query: &Query) -> RdbcResult<Option<RdbcOrmRow>> {
        Ok(None)
    }
    async fn select_list_by_sql(
        &self,
        query: &str,
        params: &[RdbcValue],
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }

    async fn execute_insert(&self, delete: &Insert) -> RdbcResult<u64> {
        Ok(0)
    }

    async fn execute_update(&self, delete: &Update) -> RdbcResult<u64> {
        Ok(0)
    }

    async fn execute_delete(&self, delete: &Delete) -> RdbcResult<u64> {
        Ok(0)
    }
}
