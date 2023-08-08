use bmbp_app_common::{BmbpHashMap, BmbpValue};

/// DataBase 数据库实例对像
pub struct DataBase {}
/// DataSource 数据源对象
pub struct DataSource {}
/// Connection 数据库连接对象
pub struct Connection {}
/// ConnectionPool 数据库连接池对象
pub struct ConnectionPool {}
///Transtraction 数据库事务对象
pub struct Transtraction {}

///RowRecords 数据库行记录对象组
pub struct RowRecords {
    row_count: usize,
    record: Vec<RowRecord>,
}

///RowRecord 数据库行记录对象
pub struct RowRecord {
    columns: BmbpHashMap,
}

///ColumnRecord 数据库列对象
pub struct ColumnRecord {
    column_name: String,
    column_type: String,
    column_value: Option<BmbpValue>,
}
