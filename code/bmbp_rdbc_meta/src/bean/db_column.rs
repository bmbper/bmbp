use crate::bean::db_type::BmbpColumnDefaultValue;
use crate::BmbpColumnDataType;
use bmbp_marco::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcColumn {
    owner_user: Option<String>,
    owner_schema: Option<String>,
    owner_table: Option<String>,
    column_name: Option<String>,
    column_comment: Option<String>,
    column_data_type: Option<BmbpColumnDataType>,
    column_length: Option<i32>,
    column_scale: Option<i32>,
    column_default_value: Option<BmbpColumnDefaultValue>,
    column_nullable: Option<bool>,
    column_primary: Option<bool>,
    column_unique: Option<bool>,
}
