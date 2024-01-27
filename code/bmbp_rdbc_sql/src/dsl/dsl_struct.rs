use crate::{Query, RdbcFunc, RdbcSQL, RdbcValue};
use crate::RdbcColumn::Value;

/// RdbcColumn SELECT 返回列
pub enum RdbcColumn {
    Table(RdbcTableColumn),
    Query(RdbcQueryColumn),
    Func(RdbcFuncColumn),
    Value(RdbcValueColumn),
}
impl RdbcSQL for RdbcColumn{
    fn to_sql(&self) -> String {
        match self {
            RdbcColumn::Table(_) => "".to_string(),
            RdbcColumn::Query(_) => "".to_string(),
            RdbcColumn::Func(_) => "".to_string(),
            RdbcColumn::Value(_) => "".to_string(),
        }
    }
}
pub struct RdbcTableColumn {
    schema_: Option<String>,
    table_: Option<String>,
    name_: String,
    alias_: Option<String>,
}

pub struct RdbcValueColumn {
    name_: RdbcValue,
    alias_: Option<String>,
}

pub struct RdbcFuncColumn {
    columns_: RdbcFunc,
    alias_: Option<String>,
}

pub struct RdbcQueryColumn {
    name_: Query,
    alias_: Option<String>,
}

pub enum RdbcTable {
    Table(RdbcSchemaTable),
    Query(RdbcQueryTable),
}

pub enum RdbcTableJoinType {
    Left,
    Right,
    Inner,
    Full,
}

pub struct RdbcSchemaTable {
    schema_: Option<String>,
    name_: String,
    alias_: Option<String>,
    join_: Option<RdbcTableJoinType>,
    filter_: Option<RdbcFilter>,
}

pub struct RdbcQueryTable {
    name_: Query,
    alias_: Option<String>,
    join_: Option<RdbcTableJoinType>,
    filter_: Option<RdbcFilter>,
}

pub enum RdbcConcatType {
    And,
    Or,
}

pub struct RdbcFilter {
    concat_: RdbcConcatType,
    compare: Vec<RdbcFilterColumn>,
}

pub enum RdbcFilterColumn {
    Table(RdbcTableFilterColumn),
    Func(RdbcFuncFilterColumn),
    Query(RdbcQueryFilterColumn),
    Filter(RdbcFilter),
}

pub struct RdbcTableFilterColumn {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
}

pub struct RdbcFuncFilterColumn {
    column_: RdbcFunc,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
}

pub struct RdbcQueryFilterColumn {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<Query>,
}

pub enum RdbcCompareType {
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Exists,
    NotExists,
}

pub enum RdbcOrder {
    Column(RdbcColumnOrder)
}

pub struct RdbcColumnOrder {
    column: RdbcColumn,
    order: RdbcOrderType,
}

pub enum RdbcOrderType {
    Asc,
    Desc,
}
