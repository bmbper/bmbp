use crate::vars::global::DATA_BASE_TYPE;

/// 获取数据库类型
pub fn global_db_type() -> String {
    DATA_BASE_TYPE.read().unwrap().clone()
}
