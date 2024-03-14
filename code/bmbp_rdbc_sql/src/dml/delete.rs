use std::collections::HashMap;
use crate::{DatabaseType, Query, RdbcColumn, RdbcConcatType, RdbcFilterInner, RdbcOrder, RdbcFilter, RdbcSQL, RdbcTable, RdbcTableInner, RdbcTableJoinType, RdbcValue, Update};

pub struct Delete {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTableInner>,
    join_: Option<Vec<RdbcTableInner>>,
    filter_: Option<RdbcFilterInner>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<RdbcFilterInner>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<HashMap<String, RdbcValue>>,
}
impl Delete {
    pub fn new() -> Delete {
        Delete {
            driver_: None,
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
    pub fn driver(driver: DatabaseType) -> Self {
        Delete {
            driver_: Some(driver),
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

impl RdbcTable for Delete {}
impl RdbcFilter for Delete {
    fn init_filter(&mut self) -> &mut Self {
        self
    }

    fn get_filter_mut(&mut self) -> &mut RdbcFilterInner {
        todo!()
    }

    fn add_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self {
        self
    }
}

impl RdbcSQL for Delete {
    fn to_sql(&self) -> String {
        "".to_string()
    }

    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}