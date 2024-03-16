use std::collections::HashMap;

use crate::RdbcValue;

pub(crate) fn base_build_sql(
    tag: &str,
    sql: String,
    params: HashMap<String, RdbcValue>,
) -> (String, Vec<RdbcValue>) {
    let (mut pg_sql, mut page_params) = ("".to_string(), vec![]);
    for (index, (key, value)) in params.iter().enumerate() {
        pg_sql = sql.replace(
            format!("${{{}}}", key).as_str(),
            format!("{}{}", tag, index + 1).as_str(),
        );
        page_params.push(value.clone());
    }
    (pg_sql, page_params)
}
