use crate::bean::db_type::{ColumnDataType, DefaultValue};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcColumnVo {
    name: String,
    comment: Option<String>,
    data_type: ColumnDataType,
    length: Option<i32>,
    scale: Option<i32>,
    nullable: Option<bool>,
    default_value: Option<DefaultValue>,
    primary_key: Option<bool>,
    unique: Option<bool>,
}
