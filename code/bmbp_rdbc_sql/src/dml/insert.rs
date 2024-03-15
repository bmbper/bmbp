use crate::{
    DatabaseType, Delete, Query, RdbcDmlValue, RdbcFilter, RdbcSQL, RdbcTable, RdbcTableInner,
    RdbcValue,
};
use std::collections::HashMap;

pub struct Insert {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTableInner>,
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
    pub fn insert_table<T>(&mut self, table: T) -> &mut Self
    where
        T: ToString,
    {
        self.table_.push(RdbcTableInner::table(table));
        self
    }
    pub fn insert_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
    where
        T: ToString,
    {
        self.table_
            .push(RdbcTableInner::schema_table(schema, table));
        self
    }
    pub fn insert_select_query(&mut self, query: Query) -> &mut Self {
        self.query_ = Some(query);
        self
    }

    pub fn insert_column<T>(&mut self, column: T) -> &mut Self
    where
        T: ToString,
    {
        if self.column_.is_none() {
            self.column_ = Some(vec![column.to_string()]);
        } else {
            self.column_.as_mut().unwrap().push(column.to_string());
        }
        self
    }
    pub fn insert_value<T>(&mut self, column: T) -> &mut Self
    where
        T: ToString,
    {
        let value_ = RdbcDmlValue::VALUE(RdbcValue::String(column.to_string()));
        if self.values_.is_none() {
            self.values_ = Some(vec![value_]);
        } else {
            self.values_.as_mut().unwrap().push(value_);
        }
        self
    }
    fn init_column_values(&mut self) {
        if self.column_values.is_none() {
            let mut map = HashMap::new();
            self.column_values = Some(map);
        }
    }

    pub fn insert_column_value<T, V>(&mut self, column: T, value: V) -> &mut Self
    where
        T: ToString,
        V: ToString,
    {
        self.init_column_values();
        self.column_values.as_mut().unwrap().insert(
            column.to_string(),
            RdbcDmlValue::VALUE(RdbcValue::String(value.to_string())),
        );
        self
    }

    pub fn insert_op_column_value<T, V>(&mut self, column: T, value: Option<V>) -> &mut Self
    where
        T: ToString,
        V: ToString,
    {
        self.init_column_values();
        let rdbc_value = match value {
            Some(v) => RdbcValue::String(v.to_string()),
            None => RdbcValue::Null,
        };
        self.column_values
            .as_mut()
            .unwrap()
            .insert(column.to_string(), RdbcDmlValue::VALUE(rdbc_value));
        self
    }
}
impl RdbcTable for Insert {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        self.table_.as_mut()
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
