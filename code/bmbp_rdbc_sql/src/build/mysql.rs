use std::collections::HashMap;

use crate::{Delete, Insert, Query, RdbcValue, Update};
use crate::build::base::base_build_sql;
use crate::build::vars::MYSQL_PARAMS_TAG;

pub fn mysql_build_sql(
    sql: String,
    params: HashMap<String, RdbcValue>,
) -> (String, Vec<RdbcValue>) {
    base_build_sql(MYSQL_PARAMS_TAG, sql, params)
}
pub fn mysql_build_query_script(query: &Query) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_insert_script(insert: &Insert) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_update_script(update: &Update) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_delete_script(delete: &Delete) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
