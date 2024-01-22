use std::collections::HashMap;
use std::fmt::{Display};
use crate::{RdbcSQLCompare, RdbcSQLFilter, RdbcSQLValue};

#[derive(Debug, Clone)]
pub enum RdbcValue {
    String(String),
    Int(i32),
    BigInt(i128),
    Float(f32),
    BigFloat(f64),
    Bool(bool),
    Map(HashMap<String, RdbcValue>),
    Array(Vec<RdbcValue>),
    NULL,
}

impl Display for RdbcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "".to_string())
    }
}

impl RdbcSQLValue for RdbcValue {
    fn to_value(&self) -> String {
        match self {
            RdbcValue::String(s) => format!("'{}'", s),
            RdbcValue::Int(i) => i.to_string(),
            RdbcValue::BigInt(i) => i.to_string(),
            RdbcValue::Float(f) => f.to_string(),
            RdbcValue::BigFloat(f) => f.to_string(),
            RdbcValue::Bool(b) => b.to_string(),
            RdbcValue::Map(m) => "".to_string(),
            RdbcValue::Array(a) => "".to_string(),
            RdbcValue::NULL => "".to_string(),
        }
    }
}


pub enum RdbcCompareType {
    EQ,
    NE,
    GT,
    GE,
    LT,
    LE,
}

impl RdbcSQLCompare for RdbcCompareType {
    fn to_compare(&self) -> String {
        match self {
            RdbcCompareType::EQ => "=".to_string(),
            RdbcCompareType::NE => "!=".to_string(),
            RdbcCompareType::GT => ">".to_string(),
            RdbcCompareType::GE => ">=".to_string(),
            RdbcCompareType::LT => "<".to_string(),
            RdbcCompareType::LE => "<=".to_string(),
        }
    }
}

pub enum RdbcSQLCompareConcat {
    AND,
    OR,
}

impl RdbcSQLFilter for RdbcSQLCompareConcat {
    fn to_filter(&self) -> String {
        match self {
            RdbcSQLCompareConcat::AND => " AND ".to_string(),
            RdbcSQLCompareConcat::OR => " OR ".to_string(),
        }
    }
}