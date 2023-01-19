// db_escape 数据库转义，表名、列名
pub fn db_escape(filed: &String) -> String {
    filed.clone()
}

// db_escape 数据库別名转义
pub fn db_alias_escape(filed: &String) -> String {
    format!("\"{}\"", filed)
}

// db_escape 常量转义
pub fn db_const_escape(filed: &String) -> String {
    format!("'{}'", filed)
}
