use crate::BmbpRdbcTable;
use bmbp_bean_marco::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcSchema {
    schema_name: Option<String>,
    schema_comment: Option<String>,
    schema_owner: Option<String>,
    schema_charset: Option<String>,
    schema_tables: Option<Vec<BmbpRdbcTable>>,
}
