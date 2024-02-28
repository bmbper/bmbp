use std::collections::HashMap;
use crate::{DatabaseType, RdbcColumn, RdbcFilter, RdbcDmlValue, RdbcOrder, RdbcTable, RdbcValue};

pub struct Update{
    driver_: Option<DatabaseType>,
    set_values_: Option<HashMap<String, RdbcDmlValue>>,
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