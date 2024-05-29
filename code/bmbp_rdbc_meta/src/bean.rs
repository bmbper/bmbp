use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcSchemaVo {
    name: String,
    comment: Option<String>,
    owner: Option<String>,
    charset: Option<String>,
    tables: Vec<RdbcTableVo>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableVo {
    name: String,
    comment: Option<String>,
    charset: Option<String>,
    schema: Option<String>,
    columns: Vec<RdbcColumnVo>,
    primary_key: Option<RdbcTablePrimaryKeyVo>,
    foreign_key: Option<Vec<RdbcTableForeignKeyVo>>,
    check: Option<Vec<RdbcTableCheckVo>>,
    index: Option<Vec<RdbcTableIndexVo>>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcColumnVo {
    name: String,
    comment: Option<String>,
    data_type: RdbcDataType,
    length: Option<i32>,
    scale: Option<i32>,
    nullable: Option<bool>,
    default_value: Option<String>,
    primary_key: Option<bool>,
    unique: Option<bool>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTablePrimaryKeyVo {
    name: String,
    comment: Option<String>,
    columns: Vec<String>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableForeignKeyVo {
    name: String,
    comment: Option<String>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableCheckVo {
    name: String,
    comment: Option<String>,
    target_table: Option<String>,
    columns: Vec<(String, String)>,
    delete_rule: Option<String>,
    update_rule: Option<String>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableIndexVo {
    name: String,
    comment: Option<String>,
    columns: Vec<String>,
    unique: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum RdbcDataType {
    CHAR,
    #[default]
    STRING,
    TEXT,
    INT,
    LONG,
    DECIMAL,
    FLOAT,
    DATE,
    TIME,
    DateTime,
    Timestamp,
    Boolean,
}
