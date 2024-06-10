use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum BmbpDBType {
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
pub enum BmbpColumnDataType {
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
pub enum BmbpColumnDefaultValue {
    VALUE(String),
    FUNCTION(String),
}
