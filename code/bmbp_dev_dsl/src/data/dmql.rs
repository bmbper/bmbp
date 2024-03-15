// DML DQL
// DML (Data Manipulation Language) 数据操纵语言, 用于对数据库进行增删改查操作
// DQL (Data Query Language) 数据查询语言, 用于对数据库进行查询操作

use crate::data::ddl::BmbpDBColumn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsetTable {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表字段
    columns: Vec<BmbpDBDmlColumn>,
}
impl InsetTable {
    pub fn new(
        schema_code: String,
        table_code: String,
        columns: Vec<BmbpDBDmlColumn>,
    ) -> InsetTable {
        InsetTable {
            schema_code,
            table_code,
            columns,
        }
    }
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_columns(&self) -> Vec<BmbpDBDmlColumn> {
        self.columns.clone()
    }
    pub fn get_column_names(&self) -> Vec<String> {
        let mut column_names = vec![];
        for column in self.columns.iter() {
            column_names.push(column.column.get_column_name());
        }
        column_names
    }
    pub fn get_column_values(&self) -> Vec<BmbpDmlColumnValue> {
        self.columns
            .iter()
            .map(|column| column.value.clone())
            .collect()
    }
}
impl Into<String> for InsetTable {
    fn into(self) -> String {
        format!(
            "INSERT INTO {}.{} ({}) VALUES ({})",
            self.schema_code,
            self.table_code,
            self.columns.join(","),
            self.columns.join(",")
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTable {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表字段
    columns: Vec<BmbpDBDmlColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpDBDmlColumn {
    // 数据库表字段
    column: BmbpDBColumn,
    // 数据库表字段值
    value: BmbpDmlColumnValue,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpDmlColumnValue {
    // 字符串
    String(String),
    // 整数
    Integer(i32),
    // 布尔
    Boolean(bool),
    // 日期
    Date(String),
    // 时间
    Time(String),
    // 日期时间
    DateTime(String),
    // 二进制
    Binary(Vec<u8>),
    // SQL
    Sql(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTable {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    filter: Option<BmbpDBQueryFilterPart>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectTable {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表字段
    columns: Vec<BmbpDBQuerySelectColumn>,
    filter: Option<BmbpDBQueryFilterPart>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpDBQuerySelectColumn {
    typ: BmbpDBQueryColumnType,
    alias: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpDBQueryColumnType {
    // Column
    Column(BmbpDBColumn),
    // ColumnName
    ColumnName(String),
    // 字符串
    String(String),
    // 整数
    Integer(i32),
    // 双精度浮点数
    Double(f64),
    // 布尔
    Boolean(bool),
    // 日期
    Date(chrono::NaiveDate),
    // 时间
    Time(chrono::NaiveTime),
    // 日期时间
    DateTime(chrono::NaiveDateTime),
    // SQL
    Sql(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpDBQueryFilter {
    And(Vec<BmbpDBQueryFilterPart>),
    OR(Vec<BmbpDBQueryFilterPart>),
    AndQuote(Vec<BmbpDBQueryFilter>),
    OrQuote(Vec<BmbpDBQueryFilter>),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpDBQueryFilterPart {
    column: BmbpDBQueryFilterColumn,
    value: BmbpDmlColumnValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpDBQueryFilterColumn {
    // Column
    Column(BmbpDBColumn),
    // ColumnName
    ColumnName(String),
    // 字符串
    String(String),
    // 整数
    Integer(i32),
    // 双精度浮点数
    Double(f64),
    // 布尔
    Boolean(bool),
    // 日期
    Date(chrono::NaiveDate),
    // 时间
    Time(chrono::NaiveTime),
    // 日期时间
    DateTime(chrono::NaiveDateTime),
    // SQL
    Sql(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpDBQueryFilterColumnType {
    // 等于
    Equal,
    // 不等于
    NotEqual,
    // 大于
    Greater,
    // 小于
    Less,
    // 大于等于
    GreaterEqual,
    // 小于等于
    LessEqual,
    // 包含
    In,
    // 不包含
    NotIn,
    // 开始于
    StartWith,
    // 结束于
    EndWith,
    // 包含
    Like,
    // 不包含
    NotLike,
}
