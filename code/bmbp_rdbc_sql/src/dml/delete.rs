use std::collections::HashMap;
use crate::{DatabaseType, RdbcColumn, RdbcFilter, RdbcOrder, RdbcTable, RdbcValue};

pub struct Delete {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTable>,
    join_: Option<Vec<RdbcTable>>,
    filter_: Option<RdbcFilter>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<RdbcFilter>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<HashMap<String, RdbcValue>>,
}