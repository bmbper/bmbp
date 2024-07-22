use bmbp_bean_marco::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcTablePrimaryKey {
    owner_user: Option<String>,
    owner_schema: Option<String>,
    owner_table: Option<String>,
    primary_name: String,
    primary_comment: Option<String>,
    primary_columns: Vec<String>,
}
