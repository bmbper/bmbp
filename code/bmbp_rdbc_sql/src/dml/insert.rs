use std::collections::HashMap;
use crate::{DatabaseType, Query, RdbcDmlValue, RdbcTable};

pub struct Insert {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTable>,
    column_: Option<Vec<String>>,
    values_: Option<Vec<RdbcDmlValue>>,
    column_values: Option<HashMap<String, RdbcDmlValue>>,
    query_: Option<Query>,
}
