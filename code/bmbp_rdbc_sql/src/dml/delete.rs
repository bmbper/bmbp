use std::collections::HashMap;

use crate::{
    DatabaseType, RdbcColumn, RdbcConcatType, RdbcFilter, RdbcFilterInner, RdbcOrder,
    RdbcTable, RdbcTableInner, RdbcValue,
};

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

impl RdbcTable for Delete {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        self.table_.as_mut()
    }
}

impl RdbcFilter for Delete {
    fn init_filter(&mut self) -> &mut Self {
        if self.filter_.is_none() {
            self.filter_ = Some(RdbcFilterInner::new());
        }
        self
    }
    fn get_filter_mut(&mut self) -> &mut RdbcFilterInner {
        self.init_filter();
        self.filter_.as_mut().unwrap()
    }
    fn with_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self {
        let filter_ = {
            if self.filter_.is_some() {
                RdbcFilterInner::concat_with_filter(concat_type, self.filter_.take().unwrap())
            } else {
                RdbcFilterInner::concat(concat_type)
            }
        };
        self.filter_ = Some(filter_);
        self
    }
}
