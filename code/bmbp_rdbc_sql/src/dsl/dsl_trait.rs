use std::iter::Filter;
use serde::de::Unexpected::Option;
use crate::{Delete, Insert, Query, RdbcColumn, RdbcConcatType, RdbcFilter, RdbcFilterItem, RdbcValue, Update, value_column};

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

pub trait RdbcQueryFilter {
    fn init_filter(&mut self) -> &mut Self;
    fn get_filter_mut(&mut self) -> &mut RdbcFilter;
    fn add_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self;
    fn and(&mut self) -> &mut Self {
        self.add_filter(RdbcConcatType::And);
        self
    }
    fn or(&mut self) -> &mut Self {
        self.add_filter(RdbcConcatType::And);
        self
    }
}