use std::collections::HashMap;
use crate::RdbcValue;

pub trait RdbcSQL {
    fn to_sql(&self) -> String;
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

impl<T> RdbcSQL for T where T: ToString {
    fn to_sql(&self) -> String {
        self.to_string()
    }
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        (self.to_string(), vec![])
    }
}
