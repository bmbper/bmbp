use std::collections::HashMap;
use std::sync::RwLock;

use crate::build::{mysql_build_insert_script, pg_build_insert_script};
use crate::{
    DatabaseType, Query, RdbcDmlValue, RdbcSQL, RdbcTable, RdbcTableColumn, RdbcTableInner,
    RdbcValue,
};

pub struct Insert {
    driver_: RwLock<Option<DatabaseType>>,
    table_: Vec<RdbcTableInner>,
    column_: Vec<RdbcTableColumn>,
    values_: Vec<RdbcDmlValue>,
    column_values: Vec<(RdbcTableColumn, RdbcDmlValue)>,
    query_: Option<Query>,
}

impl Insert {
    pub fn new() -> Insert {
        Insert {
            driver_: RwLock::new(None),
            table_: Vec::new(),
            column_: Vec::new(),
            values_: Vec::new(),
            column_values: Vec::new(),
            query_: None,
        }
    }
    pub fn set_driver(&self, driver: DatabaseType) -> &Self {
        *self.driver_.write().unwrap() = Some(driver);
        self
    }
}

impl Insert {
    pub fn insert_query(&mut self, query: Query) -> &mut Self {
        self.query_ = Some(query);
        self
    }

    pub fn insert_column<TC>(&mut self, column: TC) -> &mut Self
    where
        RdbcTableColumn: From<TC>,
    {
        self.column_.push(RdbcTableColumn::from(column));
        self
    }
    pub fn insert_value<RV>(&mut self, value: RV) -> &mut Self
    where
        RdbcDmlValue: From<RV>,
    {
        self.values_.push(RdbcDmlValue::from(value));
        self
    }

    pub fn insert_column_value<TC, RV>(&mut self, column: TC, value: RV) -> &mut Self
    where
        RdbcTableColumn: From<TC>,
        RdbcDmlValue: From<RV>,
    {
        self.column_values
            .push((RdbcTableColumn::from(column), RdbcDmlValue::from(value)));
        self
    }
}
impl RdbcTable for Insert {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        self.table_.as_mut()
    }
}

impl RdbcSQL for Insert {
    fn build_script(&self, database_type: DatabaseType) -> (String, HashMap<String, RdbcValue>) {
        match database_type {
            DatabaseType::Postgres => pg_build_insert_script(self),
            DatabaseType::MySQL => mysql_build_insert_script(self),
        }
    }
}
