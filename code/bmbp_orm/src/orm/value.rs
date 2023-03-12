use std::any::Any;
use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{Format, IsNull, ToSql, Type};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BmbpValue {
    String(String),
    Int(i16),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    Bool(bool),
    Empty,
    NULL,
    Map(HashMap<String, BmbpValue>),
    Array(Vec<BmbpValue>),
}

impl BmbpValue {
    pub fn from_str(value: &str) -> Self {
        BmbpValue::String(value.to_string())
    }
    pub fn from_string(value: String) -> Self {
        BmbpValue::String(value)
    }
    pub fn from_string_ref(value: &String) -> Self {
        BmbpValue::String(value.to_string())
    }
    pub fn from_i8(value: i8) -> Self {
        BmbpValue::Int(value as i16)
    }
    pub fn from_i16(value: i16) -> Self {
        BmbpValue::Int(value as i16)
    }

    pub fn v_type(&self) -> String {
        match self {
            BmbpValue::String(_) => "string".to_string(),
            BmbpValue::Int(_) => "int".to_string(),
            BmbpValue::BigInt(_) => "bigInt".to_string(),
            BmbpValue::Float(_) => "float".to_string(),
            BmbpValue::BigFloat(_) => "bigFloat".to_string(),
            BmbpValue::Bool(_) => "bool".to_string(),
            BmbpValue::Empty => "empty".to_string(),
            BmbpValue::NULL => "null".to_string(),
            BmbpValue::Map(_) => "map".to_string(),
            BmbpValue::Array(_) => "array".to_string(),
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            BmbpValue::String(_) => true,
            _ => false,
        }
    }
    pub fn raw_string(&self) -> Option<&String> {
        match self {
            BmbpValue::String(v) => Some(v),
            _ => None,
        }
    }
    pub fn raw_int(&self) -> Option<&i16> {
        match self {
            BmbpValue::Int(v) => Some(v),
            _ => None,
        }
    }
    pub fn raw_big_int(&self) -> Option<&i64> {
        match self {
            BmbpValue::BigInt(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw_float(&self) -> Option<&f32> {
        match self {
            BmbpValue::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw_big_float(&self) -> Option<&f64> {
        match self {
            BmbpValue::BigFloat(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw_bool(&self) -> Option<&bool> {
        match self {
            BmbpValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw_map(&self) -> Option<&BmbpMap> {
        match self {
            BmbpValue::Map(v) => Some(v),
            _ => None,
        }
    }
    pub fn raw_array(&self) -> Option<&BmbpVec> {
        match self {
            BmbpValue::Array(v) => Some(v),
            _ => None,
        }
    }
}

pub type BmbpVec = Vec<BmbpValue>;
pub type BmbpMap = HashMap<String, BmbpValue>;

impl PartialEq<Self> for BmbpValue {
    fn eq(&self, other: &Self) -> bool {
        if self.v_type().eq(&other.v_type()) {
            match self {
                BmbpValue::String(v) => v.eq(other.raw_string().unwrap()),
                BmbpValue::Int(v) => v.eq(other.raw_int().unwrap()),
                BmbpValue::BigInt(v) => v.eq(other.raw_big_int().unwrap()),
                BmbpValue::Float(v) => v.eq(other.raw_float().unwrap()),
                BmbpValue::BigFloat(v) => v.eq(other.raw_big_float().unwrap()),
                BmbpValue::Bool(v) => v.eq(other.raw_bool().unwrap()),
                BmbpValue::Empty => true,
                BmbpValue::NULL => true,
                BmbpValue::Map(mp) => {
                    let other_mp = other.raw_map().unwrap();
                    if mp.keys().len().eq(&other_mp.len()) {
                        let mut eq_res = true;
                        for (key, value) in mp {
                            if other_mp.contains_key(key) {
                                if !value.eq(other_mp.get(key).unwrap()) {
                                    eq_res = false;
                                    break;
                                }
                            } else {
                                eq_res = false;
                                break;
                            }
                        }
                        eq_res
                    } else {
                        false
                    }
                }
                BmbpValue::Array(v) => {
                    let other_array = other.raw_array().unwrap();
                    if v.len().eq(&other_array.len()) {
                        let mut eq_res = true;
                        for (idx, v_item) in v.as_slice().into_iter().enumerate() {
                            if !other_array.get(idx).unwrap().eq(v_item) {
                                eq_res = false;
                                break;
                            }
                        }
                        eq_res
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio_postgres::types::ToSql;

    use crate::orm::value::{BmbpMap, BmbpValue};

    #[test]
    fn test_bmbp_value() {
        let v = BmbpValue::from_str("xxxx");
        println!("{}", serde_json::to_string(&v).unwrap());

        let mut bp = BmbpMap::new();
        bp.insert("a".to_string(), BmbpValue::from_str("xxx"));

        println!("{}", serde_json::to_string(&bp).unwrap());
    }
}
