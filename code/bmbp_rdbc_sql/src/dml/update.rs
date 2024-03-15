use std::collections::HashMap;

use crate::{
    DatabaseType, RdbcColumn, RdbcConcatType, RdbcDmlValue, RdbcFilter, RdbcFilterInner, RdbcOrder,
    RdbcSQL, RdbcTable, RdbcTableInner, RdbcValue,
};

pub struct Update {
    driver_: Option<DatabaseType>,
    set_values_: Vec<(RdbcColumn, Option<RdbcDmlValue>)>,
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

impl Update {
    pub fn new() -> Update {
        Update {
            driver_: None,
            set_values_: vec![],
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
            set_values_: vec![],
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
    pub fn set<SC, RV>(&mut self, column: SC, value: RV) -> &mut Self
        where
            RdbcColumn: From<SC>,
            RdbcDmlValue: From<RV>,
    {
        self.set_values_.push((RdbcColumn::from(column), Some(RdbcDmlValue::from(value))));
        self
    }
}

impl RdbcTable for Update {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        self.table_.as_mut()
    }
}

impl RdbcFilter for Update {
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

impl RdbcSQL for Update {
    fn to_sql(&self) -> String {
        "".to_string()
    }

    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}
