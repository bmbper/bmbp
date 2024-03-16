use std::collections::HashMap;
use std::sync::RwLock;

use crate::{
    DatabaseType, RdbcColumn, RdbcConcatType, RdbcDmlValue, RdbcFilter, RdbcFilterInner, RdbcOrder,
    RdbcSQL, RdbcTable, RdbcTableInner, RdbcValue,
};
use crate::build::{
    mysql_build_update_script, pg_build_update_script,
};

pub struct Update {
    driver_: RwLock<Option<DatabaseType>>,
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
            driver_: RwLock::new(None),
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
    pub fn set_driver(&self, driver: DatabaseType) -> &Self {
        *self.driver_.write().unwrap() = Some(driver);
        self
    }
}

impl Update {
    pub fn set<SC, RV>(&mut self, column: SC, value: RV) -> &mut Self
    where
        RdbcColumn: From<SC>,
        RdbcDmlValue: From<RV>,
    {
        self.set_values_
            .push((RdbcColumn::from(column), Some(RdbcDmlValue::from(value))));
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
    fn build_script(&self, database_type: DatabaseType) -> (String, HashMap<String, RdbcValue>) {
        match database_type {
            DatabaseType::Postgres => pg_build_update_script(self),
            DatabaseType::MySQL => mysql_build_update_script(self),
        }
    }
}
