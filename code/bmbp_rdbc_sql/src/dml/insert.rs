use std::collections::HashMap;
use crate::{DatabaseType, Query, RdbcDmlValue, RdbcSQL, RdbcTable, RdbcValue};

pub struct Insert {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTable>,
    column_: Option<Vec<String>>,
    values_: Option<Vec<RdbcDmlValue>>,
    column_values: Option<HashMap<String, RdbcDmlValue>>,
    query_: Option<Query>,
}

impl Insert {
    pub fn new() -> Insert {
        Insert {
            driver_: None,
            table_: Vec::new(),
            column_: None,
            values_: None,
            column_values: None,
            query_: None,
        }
    }
}

impl Insert {
    pub fn set_driver(&mut self, driver: DatabaseType) -> &mut Self {
        self.driver_ = Some(driver);
        self
    }
    pub fn insert_into_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table(table));
        self
    }
    pub fn insert_into_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table(schema, table));
        self
    }
    pub fn insert_select_query(&mut self, query: Query) -> &mut Self {
        self.query_ = Some(query);
        self
    }

    pub fn insert_column<T>(&mut self, column: T) -> &mut Self where T: ToString {
        if self.column_.is_none() {
            self.column_ = Some(vec![column.to_string()]);
        } else {
            self.column_.as_mut().unwrap().push(column.to_string());
        }
        self
    }
    pub fn insert_value<T>(&mut self, column: T) -> &mut Self where T: ToString {
        let value_ = RdbcDmlValue::VALUE(RdbcValue::String(column.to_string()));
        if self.values_.is_none() {
            self.values_ = Some(vec![value_]);
        } else {
            self.values_.as_mut().unwrap().push(value_);
        }
        self
    }
    pub fn insert_column_value<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        if self.column_values.is_none() {
            let mut map = HashMap::new();
            map.insert(column.to_string(), RdbcDmlValue::VALUE(RdbcValue::String(value.to_string())));
            self.column_values = Some(map);
        } else {
            self.column_values.as_mut().unwrap().insert(column.to_string(), RdbcDmlValue::VALUE(RdbcValue::String(value.to_string())));
        }
        self
    }
}

impl RdbcSQL for Insert {
    fn to_sql(&self) -> String {
        "".to_string()
    }
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}