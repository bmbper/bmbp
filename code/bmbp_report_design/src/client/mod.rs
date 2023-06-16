mod client;
use bmbp_types::{BmbpError, BmbpResp};

use self::client::{ModelClient, MysqlModelClient};
#[allow(dead_code)]
pub struct ModelClientFactory();
impl ModelClientFactory {
    #[allow(dead_code)]
    pub fn client(db_type: String) -> BmbpResp<Box<(dyn ModelClient)>> {
        match db_type.as_str() {
            "mysql" => Ok(Box::new(MysqlModelClient::new())),
            _ => Err(BmbpError::orm("暂未支持的数据库类型".to_string())),
        }
    }
}
