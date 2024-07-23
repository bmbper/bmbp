use bmbp_marco_bean::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcTableCheck {
    owner_user: Option<String>,
    owner_schema: Option<String>,
    owner_table: Option<String>,
    check_name: Option<String>,
    check_comment: Option<String>,
    check_express: Option<String>,
}
