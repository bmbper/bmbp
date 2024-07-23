use bmbp_marco_bean::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcTableIndex {
    owner_user: Option<String>,
    owner_schema: Option<String>,
    owner_table: Option<String>,
    index_name: String,
    index_comment: Option<String>,
    index_columns: Vec<String>,
    unique: Option<bool>,
}
