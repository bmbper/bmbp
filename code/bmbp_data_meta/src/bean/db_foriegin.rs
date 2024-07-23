use bmbp_marco_bean::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcTableForeignKey {
    owner_user: Option<String>,
    owner_schema: Option<String>,
    owner_table: Option<String>,
    foreign_name: String,
    foreign_comment: Option<String>,
    foreign_columns: Vec<String>,
    target_table: Option<String>,
    delete_rule: Option<String>,
    update_rule: Option<String>,
}
