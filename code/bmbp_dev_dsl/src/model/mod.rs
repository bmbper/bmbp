use crate::model::BmbpDBColumnType::STRING;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct BmbpDBSchema {
    code: String,
    name: String,
    title: String,
    remark: String,
    charset: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct BmbpDBTable {
    schema_code: String,
    code: String,
    name: String,
    title: String,
    remark: String,
    charset: String,
    columns: Vec<BmbpDBColumn>,
    primary: Vec<BmbpDbIndex>,
    constraint: Vec<BmbpDbIndex>,
    index: Vec<BmbpDbIndex>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct BmbpDBColumn {
    schema_code: String,
    code: String,
    name: String,
    title: String,
    remark: String,
    is_primary: bool,
    is_null: bool,
    typ: BmbpDBColumnType,
    length: usize,
    scale: usize,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BmbpDBColumnType {
    STRING,
    TEXT,
    INT,
    LONG,
    NUMBER,
    BOOL,
    DATE,
    DATETIME,
    TIMESTAMP,
}
impl Default for BmbpDBColumnType {
    fn default() -> Self {
        STRING
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BmbpDbIndexType {
    PRIMARY,
    CONSTRAINT,
    INDEX,
}
impl Default for BmbpDbIndexType {
    fn default() -> Self {
        BmbpDbIndexType::INDEX
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct BmbpDbIndex {
    schema_code: String,
    table_code: String,
    typ: BmbpDbIndexType,
    name: String,
    columns: Vec<String>,
}
