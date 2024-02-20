use std::clone;
use std::fmt::Display;

#[derive(Debug)]
pub enum RdbcValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntSize(isize),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntSize(usize),
    Float32(f32),
    Float64(f64),
    String(String),
    Date(String),
    Time(String),
    DateTime(String),
    Bool(bool),
    Null,
}


impl RdbcValue {
    pub fn get_string(&self) -> String {
        match self {
            RdbcValue::Int8(i) => i.to_string(),
            RdbcValue::Int16(i) => i.to_string(),
            RdbcValue::Int32(i) => i.to_string(),
            RdbcValue::Int64(i) => i.to_string(),
            RdbcValue::Int128(i) => i.to_string(),
            RdbcValue::IntSize(i) => i.to_string(),
            RdbcValue::UInt8(i) => i.to_string(),
            RdbcValue::UInt16(i) => i.to_string(),
            RdbcValue::UInt32(i) => i.to_string(),
            RdbcValue::UInt64(i) => i.to_string(),
            RdbcValue::UInt128(i) => i.to_string(),
            RdbcValue::UIntSize(i) => i.to_string(),
            RdbcValue::Float32(i) => i.to_string(),
            RdbcValue::Float64(i) => i.to_string(),
            RdbcValue::String(i) => i.to_string(),
            RdbcValue::Date(i) => i.to_string(),
            RdbcValue::Time(i) => i.to_string(),
            RdbcValue::DateTime(i) => i.to_string(),
            RdbcValue::Bool(i) => i.to_string(),
            RdbcValue::Null => "".to_string()
        }
    }
    pub fn get_usize(&self) -> Option<usize> {
        match self {
            RdbcValue::Int8(i) => Some(i.clone() as usize),
            RdbcValue::Int16(i) => Some(i.clone() as usize),
            RdbcValue::Int32(i) => Some(i.clone() as usize),
            RdbcValue::Int64(i) => Some(i.clone() as usize),
            RdbcValue::Int128(i) => Some(i.clone() as usize),
            RdbcValue::IntSize(i) => Some(i.clone() as usize),
            RdbcValue::UInt8(i) => Some(i.clone() as usize),
            RdbcValue::UInt16(i) => Some(i.clone() as usize),
            RdbcValue::UInt32(i) => Some(i.clone() as usize),
            RdbcValue::UInt64(i) => Some(i.clone() as usize),
            RdbcValue::UInt128(i) => Some(i.clone() as usize),
            RdbcValue::UIntSize(i) => Some(i.clone() as usize),
            RdbcValue::Float32(i) => Some(i.clone() as usize),
            RdbcValue::Float64(i) => Some(i.clone() as usize),
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