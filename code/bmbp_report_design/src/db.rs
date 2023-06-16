use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const CHAR_SET_NAME: &str = "UTF8";
/// 定义数据库类型
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DataBase {
    /// 数据库/模式名称
    name: String,
    /// 中文标题
    title: String,
    /// 包含的表
    tables: Vec<Table>,
    /// 所属用户
    owner: String,
}
#[allow(dead_code)]
impl DataBase {
    pub fn new() -> Self {
        DataBase::default()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 定义表名称
pub struct Table {
    /// 所属模式
    shcema: String,
    /// 表名称
    name: String,
    /// 表备注
    title: String,
    /// 表格列
    columns: Vec<Column>,
    /// 主键
    primary: Primary,
    /// 约束
    constriants: Vec<Consitrant>,
    /// 索引
    index: Vec<Index>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Column {
    /// 所属模式
    shcema: String,
    /// 所属表
    table: String,
    /// 列名称
    name: String,
    /// 列备注
    title: String,
    /// 允许为空
    allow_null: bool,
    /// 列类型
    typ: ColumnType,
    /// 默认值
    default_value: ColumnDefaultValue,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ColumnType {
    VARCHAR(usize),
    INT(usize),
    LONG(usize),
    DECIMAL((usize, usize)),
    BOOLEAN,
    CHAR,
    TEXT,
    BLOB,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ColumnDefaultValue {
    Number(isize),
    String(String),
    Func(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Primary {
    schema: String,
    table: String,
    name: String,
    fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Consitrant {
    schema: String,
    table: String,
    name: String,
    fields: Vec<String>,
    typ: ConsitrantType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConsitrantType {
    UNIQUE,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    schema: String,
    table: String,
    name: String,
    fields: Vec<String>,
    typ: IndexType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IndexType {
    UNIQUE,
    NORMAL,
}
