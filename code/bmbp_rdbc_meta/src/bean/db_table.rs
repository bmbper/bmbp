use crate::{
    BmbpRdbcColumn, BmbpRdbcTableCheck, BmbpRdbcTableForeignKey, BmbpRdbcTableIndex,
    BmbpRdbcTablePrimaryKey,
};
use bmbp_marco_bean::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpRdbcTable {
    owner_schema: Option<String>,
    owner_user: Option<String>,
    table_name: Option<String>,
    table_comment: Option<String>,
    table_charset: Option<String>,
    table_columns: Option<Vec<BmbpRdbcColumn>>,
    table_primary_key: Option<BmbpRdbcTablePrimaryKey>,
    table_foreign_key: Option<Vec<BmbpRdbcTableForeignKey>>,
    table_check: Option<Vec<BmbpRdbcTableCheck>>,
    table_index: Option<Vec<BmbpRdbcTableIndex>>,
}
