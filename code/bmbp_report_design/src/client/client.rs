use bmbp_app_common::{BmbpError, BmbpResp};

use crate::db::DataBase;

#[allow(unused_variables)]
pub trait ModelClient {
    fn database_from(&self, database_json: String) -> BmbpResp<DataBase> {
        Err(BmbpError::orm("接口未实现"))
    }
    fn table_from(&self, database_json: String) -> BmbpResp<DataBase> {
        Err(BmbpError::orm("接口未实现"))
    }
    fn column_from(&self, database_json: String) -> BmbpResp<DataBase> {
        Err(BmbpError::orm("接口未实现"))
    }
}

pub struct MysqlModelClient {}
impl MysqlModelClient {
    pub fn new() -> Self {
        MysqlModelClient {}
    }
}

impl ModelClient for MysqlModelClient {}
