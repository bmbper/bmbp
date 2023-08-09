use bmbp_app_common::{BmbpHashMap, BmbpValue};

#[allow(dead_code)]
/// DataBase 数据库实例对像
pub struct DataBase {}
#[allow(dead_code)]
/// DataSource 数据源对象
pub struct DataSource {}
#[allow(dead_code)]
/// Connection 数据库连接对象
pub struct Connection {}
#[allow(dead_code)]
/// ConnectionPool 数据库连接池对象
pub struct ConnectionPool {}
#[allow(dead_code)]
///Transtraction 数据库事务对象
pub struct Transtraction {}
#[allow(dead_code)]
///RowRecords 数据库行记录对象组
pub struct RowRecords {
    row_count: usize,
    record: Vec<RowRecord>,
}

#[allow(dead_code)]
///RowRecord 数据库行记录对象
pub struct RowRecord {
    columns: BmbpHashMap,
}

#[allow(dead_code)]
///ColumnRecord 数据库列对象
pub struct ColumnRecord {
    column_name: String,
    column_type: String,
    column_value: Option<BmbpValue>,
}
