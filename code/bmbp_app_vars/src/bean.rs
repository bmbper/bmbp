use bmbp_rdbc::{simple_column, BmbpRdbcTable, RdbcColumn, RdbcOrmRow, RdbcQueryColumn};

pub struct BmbpVars {}
impl BmbpRdbcTable for BmbpVars {
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_columns() -> Vec<String> {
        vec![]
    }

    fn get_table_primary_key() -> String {
        "".to_string()
    }

    fn get_table_union_primary_key() -> Vec<String> {
        vec![]
    }
}
impl From<RdbcOrmRow> for BmbpVars {
    fn from(row: RdbcOrmRow) -> Self {
        BmbpVars {}
    }
}
