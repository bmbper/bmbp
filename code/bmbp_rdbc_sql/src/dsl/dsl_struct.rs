use std::collections::HashMap;
use crate::{Query, RdbcFunc, RdbcSQL, RdbcValue};

/// RdbcColumn SELECT 返回列
pub enum RdbcColumn {
    Table(RdbcTableColumn),
    Query(RdbcQueryColumn),
    Func(RdbcFuncColumn),
    Value(RdbcValueColumn),
}

impl RdbcSQL for RdbcColumn {
    fn to_sql(&self) -> String {
        match self {
            RdbcColumn::Table(t) => t.to_sql(),
            RdbcColumn::Query(_) => "".to_string(),
            RdbcColumn::Func(_) => "".to_string(),
            RdbcColumn::Value(_) => "".to_string(),
        }
    }
}

impl RdbcColumn {
    pub fn column<T>(name: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Table(RdbcTableColumn::column(name))
    }
    pub fn column_as_alias<T, E>(name: T, alias: E) -> RdbcColumn where T: ToString, E: ToString {
        RdbcColumn::Table(RdbcTableColumn::column_as_alias(name, alias))
    }
    pub fn table_column<T, C>(table: T, name: C) -> RdbcColumn where T: ToString, C: ToString {
        RdbcColumn::Table(RdbcTableColumn::table_column(table, name))
    }
    pub fn table_column_as_alias<T>(table: T, name: T, alias: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Table(RdbcTableColumn::table_column_as_alias(table, name, alias))
    }

    pub fn schema_table_column<S, T, C>(schema: S, table: T, name: C) -> RdbcColumn
        where S: ToString, T: ToString, C: ToString {
        RdbcColumn::Table(RdbcTableColumn::schema_table_column(schema, table, name))
    }
    pub fn schema_table_column_as_alias<S, T, C, A>(schema: S, table: T, name: C, alias: A) -> RdbcColumn
        where S: ToString, T: ToString, C: ToString, A: ToString {
        RdbcColumn::Table(RdbcTableColumn::schema_table_column_as_alias(schema, table, name, alias))
    }
    pub fn rdbc_value(value: RdbcValue) -> RdbcColumn {
        RdbcColumn::Value(RdbcValueColumn::rdbc_value(value))
    }

    pub fn rdbc_value_alias<T>(value: RdbcValue, alias: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Value(RdbcValueColumn::rdbc_value_alias(value, alias))
    }
    pub fn raw_value<T>(value: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Value(RdbcValueColumn::raw_value(value))
    }
    pub fn raw_value_alias<T>(value: T, alias: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Value(RdbcValueColumn::raw_value_alias(value, alias))
    }

    pub fn string_value<T>(value: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Value(RdbcValueColumn::string_value(value))
    }
    pub fn string_value_alias<T>(value: T, alias: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Value(RdbcValueColumn::string_value_alias(value, alias))
    }
    pub fn query(query: Query) -> RdbcColumn {
        RdbcColumn::Query(RdbcQueryColumn::query(query))
    }
    pub fn query_alias<T>(query: Query, alias: T) -> RdbcColumn where T: ToString {
        RdbcColumn::Query(RdbcQueryColumn::query_alias(query, alias))
    }
}

pub struct RdbcTableColumn {
    schema_: Option<String>,
    table_: Option<String>,
    name_: String,
    alias_: Option<String>,
}

impl RdbcSQL for RdbcTableColumn {
    fn to_sql(&self) -> String {
        let mut column = self.name_.clone();
        if let Some(ref alias) = self.alias_ {
            column = format!("{} AS {}", column, alias);
        }
        if let Some(ref table) = self.table_ {
            column = format!("{}.{}", table, column);
        }
        if let Some(ref schema) = self.schema_ {
            column = format!("{}.{}", schema, column);
        }
        column
    }
}

impl RdbcTableColumn {
    fn column<T>(name: T) -> RdbcTableColumn where T: ToString {
        RdbcTableColumn {
            schema_: None,
            table_: None,
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn column_as_alias<T, E>(name: T, alias: E) -> RdbcTableColumn where T: ToString, E: ToString {
        RdbcTableColumn {
            schema_: None,
            table_: None,
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
    fn table_column<T, C>(table: T, name: C) -> RdbcTableColumn where T: ToString, C: ToString {
        RdbcTableColumn {
            schema_: None,
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn table_column_as_alias<T>(table: T, name: T, alias: T) -> RdbcTableColumn where T: ToString {
        RdbcTableColumn {
            schema_: None,
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
    fn schema_table_column<S, T, C>(schema: S, table: T, name: C) -> RdbcTableColumn where S: ToString, T: ToString, C: ToString {
        RdbcTableColumn {
            schema_: Some(schema.to_string()),
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn schema_table_column_as_alias<S, T, C, A>(schema: S, table: T, name: C, alias: A) -> RdbcTableColumn
        where S: ToString, T: ToString, C: ToString, A: ToString {
        RdbcTableColumn {
            schema_: Some(schema.to_string()),
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
}

pub struct RdbcValueColumn {
    name_: RdbcValue,
    alias_: Option<String>,
}

impl RdbcValueColumn {
    fn rdbc_value(value: RdbcValue) -> RdbcValueColumn {
        RdbcValueColumn {
            name_: value,
            alias_: None,
        }
    }
    fn rdbc_value_alias<T>(value: RdbcValue, alias: T) -> RdbcValueColumn where T: ToString {
        RdbcValueColumn {
            name_: value,
            alias_: Some(alias.to_string()),
        }
    }
    fn raw_value<T>(value: T) -> RdbcValueColumn where T: ToString {
        RdbcValueColumn {
            name_: RdbcValue::String(value.to_string()),
            alias_: None,
        }
    }
    fn raw_value_alias<T>(value: T, alias: T) -> RdbcValueColumn where T: ToString {
        RdbcValueColumn {
            name_: RdbcValue::String(value.to_string()),
            alias_: Some(alias.to_string()),
        }
    }

    fn string_value<T>(value: T) -> RdbcValueColumn where T: ToString {
        RdbcValueColumn {
            name_: RdbcValue::String(format!("'{}'", value.to_string())),
            alias_: None,
        }
    }
    fn string_value_alias<T>(value: T, alias: T) -> RdbcValueColumn where T: ToString {
        RdbcValueColumn {
            name_: RdbcValue::String(format!("'{}'", value.to_string())),
            alias_: Some(alias.to_string()),
        }
    }
}

pub struct RdbcFuncColumn {
    columns_: RdbcFunc,
    alias_: Option<String>,
}

pub struct RdbcQueryColumn {
    name_: Query,
    alias_: Option<String>,
}

impl RdbcQueryColumn {
    fn query(query: Query) -> RdbcQueryColumn {
        RdbcQueryColumn {
            name_: query,
            alias_: None,
        }
    }
    fn query_alias<T>(query: Query, alias: T) -> RdbcQueryColumn where T: ToString {
        RdbcQueryColumn {
            name_: query,
            alias_: Some(alias.to_string()),
        }
    }
}

pub enum RdbcTable {
    Table(RdbcSchemaTable),
    Query(RdbcQueryTable),
}

impl RdbcSQL for RdbcTable {
    fn to_sql(&self) -> String {
        match self {
            RdbcTable::Table(ref table) => table.to_sql(),
            RdbcTable::Query(ref query) => query.to_sql(),
        }
    }
}

impl RdbcTable {
    pub(crate) fn table<T>(table: T) -> RdbcTable where T: ToString {
        RdbcTable::Table(RdbcSchemaTable::table(table))
    }
    pub fn table_alias<T>(table: T, alias: T) -> RdbcTable where T: ToString {
        RdbcTable::Table(RdbcSchemaTable::table_alias(table, alias))
    }
    pub fn schema_table<T>(schema: T, table: T) -> RdbcTable where T: ToString {
        RdbcTable::Table(RdbcSchemaTable::schema_table(schema, table))
    }
    pub fn schema_table_alias<T>(schema: T, table: T, alias: T) -> RdbcTable where T: ToString {
        RdbcTable::Table(RdbcSchemaTable::schema_table_alias(schema, table, alias))
    }
    pub(crate) fn left_join_table<T>(table: T) -> Self where T: ToString {
        RdbcTable::Table(RdbcSchemaTable::left_join_table(table))
    }
    pub fn temp_table(table: Query) -> RdbcTable {
        RdbcTable::Query(RdbcQueryTable::query(table))
    }
    pub fn temp_table_alias<T>(table: Query, alias: T) -> RdbcTable where T: ToString {
        RdbcTable::Query(RdbcQueryTable::query_alias(table, alias))
    }
    fn query(table: Query) -> RdbcTable {
        RdbcTable::Query(RdbcQueryTable::query(table))
    }
}

impl RdbcTable {
    pub fn join(&mut self, join_: RdbcTableJoinType) -> &mut Self {
        match self {
            RdbcTable::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTable::Query(ref mut table) => {}
        }
        self
    }
    pub fn left_join(&mut self) -> &mut Self {
        match self {
            RdbcTable::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTable::Query(ref mut table) => {}
        }
        self
    }
    pub fn eq<C, V>(&mut self, column: C, value: V) -> &mut Self where V: ToString, C: ToString {
        match self {
            RdbcTable::Table(ref mut table) => {
                table.eq(column, value);
            }
            RdbcTable::Query(ref mut table) => {}
        }
        self
    }
    pub fn or(&mut self) -> &mut Self {
        match self {
            RdbcTable::Table(ref mut table) => {
                table.or();
            }
            RdbcTable::Query(ref mut table) => {}
        }
        self
    }
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
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcSQL for RdbcSchemaTable {
    fn to_sql(&self) -> String {
        let mut sql = String::new();
        if let Some(ref schema) = self.schema_ {
            sql.push_str(&schema);
            sql.push_str(".");
        }
        sql.push_str(&self.name_);
        if let Some(ref alias) = self.alias_ {
            sql.push_str(" AS ");
            sql.push_str(&alias);
        }
        sql
    }
}

impl RdbcSchemaTable {
    fn table<T>(table: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(RdbcFilter::new()),
            params_: None,
        }
    }
    fn table_alias<T>(table: T, alias: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcFilter::new()),
            params_: None,
        }
    }
    fn schema_table<T>(schema: T, table: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(RdbcFilter::new()),
            params_: None,
        }
    }
    fn schema_table_alias<T>(schema: T, table: T, alias: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcFilter::new()),
            params_:None
        }
    }
    fn left_join_table<T>(table: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcFilter::new()),
            params_:None
        }
    }
    fn left_join_table_alias<T, A>(table: T, alias: A) -> RdbcSchemaTable where T: ToString, A: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcFilter::new()),
            params_:None
        }
    }
}

impl RdbcSchemaTable {
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = RdbcFilter::concat_with_filter(concat, filter);
        self.filter_ = Some(new_filter);
        self
    }
    pub fn left_join(&mut self) -> &mut Self {
        self.join_ = Some(RdbcTableJoinType::Left);
        self
    }
    pub fn eq<C, V>(&mut self, column: C, value: V) -> &mut Self where V: ToString, C: ToString {
        self.filter_.as_mut().unwrap().eq(column, value);
        self
    }

    pub fn or(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::Or);
        self
    }
    pub fn and(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::And);
        self
    }
}


pub struct RdbcQueryTable {
    name_: Query,
    alias_: Option<String>,
    join_: Option<RdbcTableJoinType>,
    filter_: Option<RdbcFilter>,
}

impl RdbcSQL for RdbcQueryTable {
    fn to_sql(&self) -> String {
        format!("({})", self.name_.to_sql())
    }
}

impl RdbcQueryTable {
    fn query(table: Query) -> RdbcQueryTable {
        RdbcQueryTable {
            name_: table,
            alias_: None,
            join_: None,
            filter_: None,
        }
    }
    fn query_alias<T>(table: Query, alias: T) -> RdbcQueryTable where T: ToString {
        RdbcQueryTable {
            name_: table,
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: None,
        }
    }
}

pub enum RdbcConcatType {
    And,
    Or,
}

pub struct RdbcFilter {
    concat_: RdbcConcatType,
    compare: Vec<RdbcFilterColumn>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcSQL for RdbcFilter {
    fn to_sql(&self) -> String {
        "".to_string()
    }
}

impl RdbcFilter {
    pub(crate) fn concat_with_filter(concat: RdbcConcatType, filter: RdbcFilter) -> RdbcFilter {
        RdbcFilter {
            concat_: concat,
            compare: vec![RdbcFilterColumn::filter(filter)],
            params_: None,
        }
    }
    pub fn eq<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        self.compare.push(RdbcFilterColumn::eq(column, value));
        self
    }
}

impl RdbcFilter {
    pub fn new() -> RdbcFilter {
        RdbcFilter {
            concat_: RdbcConcatType::And,
            compare: vec![],
            params_: None,
        }
    }
}

pub enum RdbcFilterColumn {
    Table(RdbcTableFilterColumn),
    Func(RdbcFuncFilterColumn),
    Query(RdbcQueryFilterColumn),
    Filter(RdbcFilter),
}

impl RdbcFilterColumn {
    fn filter(filter: RdbcFilter) -> RdbcFilterColumn {
        RdbcFilterColumn::Filter(filter)
    }
}

impl RdbcFilterColumn {
    pub fn eq<T, V>(column: T, value: V) -> RdbcFilterColumn where T: ToString, V: ToString {
        RdbcFilterColumn::Table(RdbcTableFilterColumn {
            column_: RdbcColumn::column(column),
            compare_: RdbcCompareType::Eq,
            value: Some(RdbcValue::String(value.to_string())),
            ignore_null: false,
        })
    }
}

pub struct RdbcTableFilterColumn {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
    ignore_null: bool,
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

impl RdbcSQL for RdbcOrder {
    fn to_sql(&self) -> String {
        "".to_string()
    }
}

pub struct RdbcColumnOrder {
    column: RdbcColumn,
    order: RdbcOrderType,
}

pub enum RdbcOrderType {
    Asc,
    Desc,
}

pub fn table<T>(table: T) -> RdbcTable where T: ToString {
    RdbcTable::table(table)
}

pub fn left_table<T>(table: T) -> RdbcTable where T: ToString {
    RdbcTable::left_join_table(table)
}

pub fn inner_table<T>(table: T) -> RdbcTable where T: ToString {
    RdbcTable::table(table)
}

pub fn right_table<T>(table: T) -> RdbcTable where T: ToString {
    RdbcTable::table(table)
}

pub fn full_table<T>(table: T) -> RdbcTable where T: ToString {
    RdbcTable::table(table)
}