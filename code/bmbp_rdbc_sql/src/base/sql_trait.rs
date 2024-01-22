pub trait RdbcSQL {
    fn to_sql(&self) -> String;
}

/// RdbcSQLTable 表定义
pub trait RdbcSQLTable {
    fn to_table(&self) -> String;
}

pub trait RdbcSQLFilter {
    fn to_filter(&self) -> String;
}

/// RdbcSQLSelectField 查询返回
pub trait RdbcSQLSelect {
    fn to_select(&self) -> String;
}

pub trait RdbcSQLInsert {
    fn to_insert(&self) -> String;
}

/// RdbcSQLCompare 对比列
pub trait RdbcSQLCompare {
    fn to_compare(&self) -> String;
}

pub trait RdbcSQLSet {
    fn to_update(&self) -> String;
}

pub trait RdbcSQLOrder {
    fn to_order(&self) -> String;
}

pub trait RdbcSQLHaving {
    fn to_having(&self) -> String;
}

pub trait RdbcSQLGroupBy {
    fn to_group_by(&self) -> String;
}

/// RdbcSQLValue 值定义
pub trait RdbcSQLValue {
    fn to_value(&self) -> String;
}

/// RdbcSQLFunc 函数定义
pub trait RdbcSQLFunc {
    fn to_func(&self) -> String;
}

