use std::collections::HashMap;
use crate::{DatabaseType, RdbcColumn, RdbcFilter, RdbcDmlValue, RdbcOrder, RdbcTable, RdbcValue};

pub struct Update {
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


impl Update {
    pub fn new() -> Update {
        Update {
            driver_: None,
            set_values_: None,
            table_: Vec::new(),
            join_: None,
            filter_: None,
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
    pub fn driver(driver_: DatabaseType) -> Self {
        Update {
            driver_: Some(driver_),
            set_values_: None,
            table_: Vec::new(),
            join_: None,
            filter_: None,
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
}

impl Update {
    pub fn set<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        let value = RdbcDmlValue::VALUE(RdbcValue::String(value.to_string()));
        let mut set_values_ = HashMap::new();
        set_values_.insert(column.to_string(), value);
        self.set_values_ = Some(set_values_);
        self
    }
}