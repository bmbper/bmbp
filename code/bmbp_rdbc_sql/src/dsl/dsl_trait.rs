use crate::{Delete, Insert, Query, RdbcValue, Update};

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

/// RdbcSQLParser 语句解析器
pub trait RdbcSQLParser {
    fn to_query(&self, query: &Query) -> (String, Vec<RdbcValue>);
    fn to_insert(&self, query: &Insert) -> (String, Vec<RdbcValue>);
    fn to_update(&self, query: &Update) -> (String, Vec<RdbcValue>);
    fn to_delete(&self, query: &Delete) -> (String, Vec<RdbcValue>);
}