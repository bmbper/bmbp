use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum DataBaseType {
    #[default]
    POSTGRESQL,
    MYSQL,
    MARIADB,
    SQLITE,
    ORACLE,
    SQLSERVER,
    DB2,
    DM,
    H2,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum ColumnDataType {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DefaultValue {
    VALUE(String),
    FUNCTION(String),
}
