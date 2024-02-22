use std::fmt::Display;
use chrono::Utc;
#[derive(Debug,Clone)]
pub enum RdbcValue {
    Int(i16),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    String(String),
    DateTime(chrono::DateTime<Utc>),
    Bool(bool),
    Null,
}

impl RdbcValue {
    pub fn get_string(&self) -> String {
        match self {
            RdbcValue::Int(i) => i.to_string(),
            RdbcValue::BigInt(i) => i.to_string(),
            RdbcValue::Float(i) => i.to_string(),
            RdbcValue::BigFloat(i) => i.to_string(),
            RdbcValue::String(i) => i.to_string(),
            RdbcValue::DateTime(i) => i.to_string(),
            RdbcValue::Bool(i) => i.to_string(),
            RdbcValue::Null => "".to_string()
        }
    }
    pub fn get_isize(&self) -> Option<isize> {
        match self {
            RdbcValue::Int(i) => Some(i.clone() as isize),
            RdbcValue::BigInt(i) => Some(i.clone() as isize),
            RdbcValue::Float(i) => Some(i.clone() as isize),
            RdbcValue::BigFloat(i) => Some(i.clone() as isize),
            _ => None
        }
    }
    pub fn get_usize(&self) -> Option<usize> {
        match self {
            RdbcValue::Int(i) => Some(i.clone() as usize),
            RdbcValue::BigInt(i) => Some(i.clone() as usize),
            RdbcValue::Float(i) => Some(i.clone() as usize),
            RdbcValue::BigFloat(i) => Some(i.clone() as usize),
            _ => None
        }
    }
}

impl Display for RdbcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.get_string();
        write!(f, "{}", str)
    }
}