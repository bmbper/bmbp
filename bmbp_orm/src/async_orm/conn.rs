use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{Map, Value};

use bmbp_types::{BmbpResp, PageInner};

use crate::BmbpDataSource;

use super::pool::BmbpConnectionPool;

#[warn(where_clauses_object_safety)]
#[async_trait]
pub trait BmbpConn {
    /// id 获取数据库连接ID
    fn id(&self) -> String;
    /// set_pool 设置数据库连接池
    fn set_pool(&mut self, pool: Arc<BmbpConnectionPool>);
    /// pool 获取数据库源配置
    fn pool(&self) -> Arc<BmbpConnectionPool>;
    ///set_data_source 设置数据库源配置
    fn set_data_source(&mut self, data_source: Arc<BmbpDataSource>);
    ///data_source 获取数据库连接池
    fn data_source(&self) -> Arc<BmbpDataSource>;

    async fn find_page(
        &mut self,
        sql: String,
        params: &[Value],
        page_no: &usize,
        page_size: &usize,
    ) -> BmbpResp<PageInner<Map<String, Value>>>;
    async fn find_list(
        &mut self,
        _sql: String,
        _params: &[Value],
    ) -> BmbpResp<Vec<Map<String, Value>>>;
    async fn find_one(
        &mut self,
        sql: String,
        params: &[Value],
    ) -> BmbpResp<Option<Map<String, Value>>>;
    async fn insert(&mut self, sql: String, params: &[Value]) -> BmbpResp<usize>;
    async fn update(&mut self, _sql: String, _params: &[Value]) -> BmbpResp<usize>;
    async fn delete(&mut self, _sql: String, _params: &[Value]) -> BmbpResp<usize>;
    async fn execute(&mut self, _sql: String, _params: &[Value]) -> BmbpResp<usize>;
    async fn execute_ddl(&mut self, _sql: String, _params: &[Value]) -> BmbpResp<usize>;
    async fn execute_dml(&mut self, _sql: String, _params: &[Value]) -> BmbpResp<usize>;
}
