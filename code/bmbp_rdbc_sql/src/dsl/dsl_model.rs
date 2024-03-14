use std::collections::HashMap;
use crate::{Query, RdbcFunc, RdbcSQL, RdbcValue};

/// RdbcColumn SELECT 返回列
pub enum RdbcColumn {
    Table(RdbcTableColumn),
    Query(RdbcQueryColumn),
    Func(RdbcFuncColumn),
    Value(RdbcValueColumn),
}

impl From<RdbcTableColumn> for RdbcColumn {
    fn from(column: RdbcTableColumn) -> Self {
        RdbcColumn::Table(column)
    }
}

impl From<RdbcQueryColumn> for RdbcColumn {
    fn from(column: RdbcQueryColumn) -> Self {
        RdbcColumn::Query(column)
    }
}

impl From<RdbcFuncColumn> for RdbcColumn {
    fn from(column: RdbcFuncColumn) -> Self {
        RdbcColumn::Func(column)
    }
}

impl From<RdbcValueColumn> for RdbcColumn {
    fn from(column: RdbcValueColumn) -> Self {
        RdbcColumn::Value(column)
    }
}

impl From<String> for RdbcColumn {
    fn from(name: String) -> Self {
        RdbcColumn::Table(RdbcTableColumn::column(name))
    }
}

impl From<&String> for RdbcColumn {
    fn from(name: &String) -> Self {
        RdbcColumn::Table(RdbcTableColumn::column(name.to_string()))
    }
}

impl From<&str> for RdbcColumn {
    fn from(name: &str) -> Self {
        RdbcColumn::Table(RdbcTableColumn::column(name.to_string()))
    }
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
    pub fn table_column_as_alias<ST, SC, SA>(table: ST, column: SC, alias: SA) -> RdbcColumn where ST: ToString, SC: ToString, SA: ToString {
        RdbcColumn::Table(RdbcTableColumn::table_column_as_alias(table, column, alias))
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


    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcColumn {
        RdbcColumn::Func(RdbcFuncColumn::concat_split(columns, split_))
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcColumn {
        RdbcColumn::Func(RdbcFuncColumn::concat(columns))
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
    fn table_column_as_alias<ST, SC, SA>(table: ST, name: SC, alias: SA) -> RdbcTableColumn where ST: ToString, SC: ToString, SA: ToString {
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
    pub fn rdbc_value(value: RdbcValue) -> RdbcValueColumn {
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

impl RdbcFuncColumn {
    fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcFuncColumn {
        RdbcFuncColumn {
            columns_: RdbcFunc::concat_split(columns, split_),
            alias_: None,
        }
    }
    fn concat(columns: Vec<RdbcColumn>) -> RdbcFuncColumn {
        RdbcFuncColumn {
            columns_: RdbcFunc::concat(columns),
            alias_: None,
        }
    }
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

pub enum RdbcTableInner {
    Table(RdbcSchemaTable),
    Query(RdbcQueryTable),
}

impl RdbcSQL for RdbcTableInner {
    fn to_sql(&self) -> String {
        match self {
            RdbcTableInner::Table(ref table) => table.to_sql(),
            RdbcTableInner::Query(ref query) => query.to_sql(),
        }
    }
}

impl RdbcTableInner {
    pub(crate) fn table<T>(table: T) -> RdbcTableInner where T: ToString {
        RdbcTableInner::Table(RdbcSchemaTable::table(table))
    }
    pub fn table_alias<T, V>(table: T, alias: V) -> RdbcTableInner where T: ToString, V: ToString {
        RdbcTableInner::Table(RdbcSchemaTable::table_alias(table, alias))
    }
    pub fn schema_table<T>(schema: T, table: T) -> RdbcTableInner where T: ToString {
        RdbcTableInner::Table(RdbcSchemaTable::schema_table(schema, table))
    }
    pub fn schema_table_alias<T>(schema: T, table: T, alias: T) -> RdbcTableInner where T: ToString {
        RdbcTableInner::Table(RdbcSchemaTable::schema_table_alias(schema, table, alias))
    }
    pub(crate) fn left_join_table<T>(table: T) -> Self where T: ToString {
        RdbcTableInner::Table(RdbcSchemaTable::left_join_table(table))
    }
    pub fn temp_table(table: Query) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::query(table))
    }
    pub fn temp_table_alias<T>(table: Query, alias: T) -> RdbcTableInner where T: ToString {
        RdbcTableInner::Query(RdbcQueryTable::query_alias(table, alias))
    }
    fn query(table: Query) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::query(table))
    }
}

impl RdbcTableInner {
    pub fn join(&mut self, join_: RdbcTableJoinType) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    pub fn left_join(&mut self) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    pub fn eq<C, V>(&mut self, column: C, value: V) -> &mut Self where V: ToString, C: ToString {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.eq(column, value);
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    pub fn or(&mut self) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.or();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }

    pub fn on_eq<T, V, E, F>(&mut self, t1: T, c1: V, t2: E, c2: F) -> &mut Self where T: ToString, E: ToString, V: ToString, F: ToString {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.eq_column(simple_column(t1, c1), simple_column(t2, c2));
            }
            RdbcTableInner::Query(ref mut table) => {
                table.eq_column(simple_column(t1, c1), simple_column(t2, c2));
            }
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
    filter_: Option<RdbcFilterInner>,
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
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
    fn table_alias<T, V>(table: T, alias: V) -> RdbcSchemaTable where T: ToString, V: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
    fn schema_table<T>(schema: T, table: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
    fn schema_table_alias<T>(schema: T, table: T, alias: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
    fn left_join_table<T>(table: T) -> RdbcSchemaTable where T: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
    fn left_join_table_alias<T, A>(table: T, alias: A) -> RdbcSchemaTable where T: ToString, A: ToString {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcFilterInner::new()),
            params_: None,
        }
    }
}

impl RdbcSchemaTable {
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = RdbcFilterInner::concat_with_filter(concat, filter);
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
    pub fn eq_column(&mut self, col: RdbcColumn, val: RdbcColumn) -> &mut Self {
        self.filter_.as_mut().unwrap().eq_column(col, val);
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
    filter_: Option<RdbcFilterInner>,
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
    pub fn eq_column(&mut self, col: RdbcColumn, val: RdbcColumn) -> &mut Self {
        self.filter_.as_mut().unwrap().eq_column(col, val);
        self
    }
}

pub enum RdbcConcatType {
    And,
    Or,
}

pub struct RdbcFilterInner {
    concat_: RdbcConcatType,
    item_: Vec<RdbcFilterItem>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcSQL for RdbcFilterInner {
    fn to_sql(&self) -> String {
        "".to_string()
    }
}

impl RdbcFilterInner {
    pub(crate) fn concat_with_filter(concat: RdbcConcatType, filter: RdbcFilterInner) -> RdbcFilterInner {
        RdbcFilterInner {
            concat_: concat,
            item_: vec![RdbcFilterItem::filter(filter)],
            params_: None,
        }
    }
    pub fn eq<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        self.item_.push(RdbcFilterItem::eq(column, value));
        self
    }
    pub fn eq_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(RdbcFilterItem::eq_column(column, value));
        self
    }
}

impl RdbcFilterInner {
    pub fn new() -> RdbcFilterInner {
        RdbcFilterInner {
            concat_: RdbcConcatType::And,
            item_: vec![],
            params_: None,
        }
    }
}

pub enum RdbcFilterItem {
    Value(RdbcValueFilterItem),
    Column(RdbcColumnFilterItem),
    Filter(RdbcFilterInner),
}

impl RdbcFilterItem {
    fn filter(filter: RdbcFilterInner) -> RdbcFilterItem {
        RdbcFilterItem::Filter(filter)
    }
}

impl RdbcFilterItem {
    pub fn eq<T, V>(column: T, value: V) -> RdbcFilterItem where T: ToString, V: ToString {
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::column(column),
            compare_: RdbcCompareType::Eq,
            value: Some(RdbcValue::String(value.to_string())),
            ignore_null: false,
        })
    }
    pub fn eq_column(column: RdbcColumn, value: RdbcColumn) -> RdbcFilterItem {
        RdbcFilterItem::Column(RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::Eq,
            value: Some(value),
        })
    }
}

pub struct RdbcValueFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
    ignore_null: bool,
}

pub struct RdbcColumnFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcColumn>,
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


pub enum RdbcDmlValue {
    VALUE(RdbcValue),
    COLUMN(RdbcColumn),
}

pub fn table<T>(table: T) -> RdbcTableInner where T: ToString {
    RdbcTableInner::table(table)
}

pub fn left_table<T>(table: T) -> RdbcTableInner where T: ToString {
    RdbcTableInner::left_join_table(table)
}

pub fn inner_table<T>(table: T) -> RdbcTableInner where T: ToString {
    RdbcTableInner::table(table)
}

pub fn right_table<T>(table: T) -> RdbcTableInner where T: ToString {
    RdbcTableInner::table(table)
}

pub fn full_table<T>(table: T) -> RdbcTableInner where T: ToString {
    RdbcTableInner::table(table)
}

pub fn table_column<T, V>(table: T, column: V) -> RdbcTableColumn where T: ToString, V: ToString {
    RdbcTableColumn::table_column(table, column)
}

pub fn simple_column<T, V>(table: T, column: V) -> RdbcColumn where T: ToString, V: ToString {
    RdbcColumn::column_as_alias(table, column)
}

pub fn value_column<V>(column: V) -> RdbcColumn where V: ToString {
    RdbcColumn::rdbc_value(RdbcValue::String(column.to_string()))
}

