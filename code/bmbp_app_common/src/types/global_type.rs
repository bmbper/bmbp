use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BmbpValue {
    String(String),
    Int(i32),
    BigInt(i128),
    Float(f32),
    BigFloat(f64),
    Bool(bool),
    Map(HashMap<String, BmbpValue>),
    Array(Vec<BmbpValue>),
    NULL,
}

impl From<String> for BmbpValue {
    fn from(value: String) -> Self {
        BmbpValue::String(value)
    }
}

impl From<&String> for BmbpValue {
    fn from(value: &String) -> Self {
        BmbpValue::String(value.clone())
    }
}

impl From<&str> for BmbpValue {
    fn from(value: &str) -> Self {
        BmbpValue::String(value.to_string())
    }
}

impl From<i8> for BmbpValue {
    fn from(value: i8) -> Self {
        BmbpValue::Int(value as i32)
    }
}

impl From<u8> for BmbpValue {
    fn from(value: u8) -> Self {
        BmbpValue::Int(value as i32)
    }
}

impl From<i32> for BmbpValue {
    fn from(value: i32) -> Self {
        BmbpValue::Int(value)
    }
}

impl From<u32> for BmbpValue {
    fn from(value: u32) -> Self {
        BmbpValue::Int(value as i32)
    }
}

impl From<u64> for BmbpValue {
    fn from(value: u64) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<i64> for BmbpValue {
    fn from(value: i64) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<usize> for BmbpValue {
    fn from(value: usize) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<isize> for BmbpValue {
    fn from(value: isize) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<u128> for BmbpValue {
    fn from(value: u128) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<i128> for BmbpValue {
    fn from(value: i128) -> Self {
        BmbpValue::BigInt(value as i128)
    }
}

impl From<f32> for BmbpValue {
    fn from(value: f32) -> Self {
        BmbpValue::Float(value)
    }
}

impl From<f64> for BmbpValue {
    fn from(value: f64) -> Self {
        BmbpValue::BigFloat(value)
    }
}

impl From<&i8> for BmbpValue {
    fn from(value: &i8) -> Self {
        BmbpValue::Int(value.clone() as i32)
    }
}

impl From<&u8> for BmbpValue {
    fn from(value: &u8) -> Self {
        BmbpValue::Int(value.clone() as i32)
    }
}

impl From<&i32> for BmbpValue {
    fn from(value: &i32) -> Self {
        BmbpValue::Int(value.clone())
    }
}

impl From<&u32> for BmbpValue {
    fn from(value: &u32) -> Self {
        BmbpValue::Int(value.clone() as i32)
    }
}

impl From<&u64> for BmbpValue {
    fn from(value: &u64) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&i64> for BmbpValue {
    fn from(value: &i64) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&usize> for BmbpValue {
    fn from(value: &usize) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&isize> for BmbpValue {
    fn from(value: &isize) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&u128> for BmbpValue {
    fn from(value: &u128) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&i128> for BmbpValue {
    fn from(value: &i128) -> Self {
        BmbpValue::BigInt(value.clone() as i128)
    }
}

impl From<&f32> for BmbpValue {
    fn from(value: &f32) -> Self {
        BmbpValue::Float(value.clone())
    }
}

impl From<&f64> for BmbpValue {
    fn from(value: &f64) -> Self {
        BmbpValue::BigFloat(value.clone())
    }
}

impl From<bool> for BmbpValue {
    fn from(value: bool) -> Self {
        BmbpValue::Bool(value)
    }
}

impl From<char> for BmbpValue {
    fn from(value: char) -> Self {
        BmbpValue::String(String::from(value))
    }
}

impl From<&bool> for BmbpValue {
    fn from(value: &bool) -> Self {
        BmbpValue::Bool(value.clone())
    }
}

impl From<&char> for BmbpValue {
    fn from(value: &char) -> Self {
        BmbpValue::String(String::from(value.clone()))
    }
}

impl<T> From<Vec<T>> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        let mut t_vec = vec![];
        for item in value {
            let v: BmbpValue = BmbpValue::from(item);
            t_vec.push(v);
        }
        BmbpValue::Array(t_vec)
    }
}

impl<T> From<&[T]> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: &[T]) -> Self {
        let mut t_vec = vec![];
        for item in value {
            let v: BmbpValue = item.clone().into();
            t_vec.push(v);
        }
        BmbpValue::Array(t_vec)
    }
}

impl<T> From<&Vec<T>> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: &Vec<T>) -> Self {
        let mut t_vec = vec![];
        for item in value {
            let v: BmbpValue = item.clone().into();
            t_vec.push(v);
        }
        BmbpValue::Array(t_vec)
    }
}

impl<T> From<Option<T>> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            None => BmbpValue::NULL,
            Some(v) => BmbpValue::from(v),
        }
    }
}

impl<T> From<HashMap<String, T>> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: HashMap<String, T>) -> Self {
        let mut mp = HashMap::new();
        for (k, v) in value.iter() {
            mp.insert(k.clone(), BmbpValue::from(v.clone()));
        }
        BmbpValue::Map(mp)
    }
}

impl<T> From<&HashMap<String, T>> for BmbpValue
where
    T: Clone,
    BmbpValue: From<T>,
{
    fn from(value: &HashMap<String, T>) -> Self {
        let mut mp = HashMap::new();
        for (k, v) in value.iter() {
            mp.insert(k.clone(), BmbpValue::from(v.clone()));
        }
        BmbpValue::Map(mp)
    }
}

impl From<Value> for BmbpValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => BmbpValue::NULL,
            Value::Bool(v) => BmbpValue::from(v),
            Value::Number(v) => {
                if v.is_f64() {
                    BmbpValue::from(v.as_f64())
                } else if v.is_i64() {
                    BmbpValue::from(v.as_i64())
                } else if v.is_u64() {
                    BmbpValue::from(v.as_u64())
                } else {
                    BmbpValue::NULL
                }
            }
            Value::String(v) => BmbpValue::from(v),
            Value::Array(v) => {
                let mut b_v = vec![];
                for item in v {
                    b_v.push(BmbpValue::from(item));
                }
                BmbpValue::Array(b_v)
            }
            Value::Object(v) => {
                let mut b_v = HashMap::new();
                for (k, item) in v {
                    b_v.insert(k, BmbpValue::from(item));
                }
                BmbpValue::Map(b_v)
            }
        }
    }
}

impl PartialEq<Self> for BmbpValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            BmbpValue::String(v) => {
                let o_v_op = other.raw_string();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::Int(v) => {
                let o_v_op = other.raw_int();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::BigInt(v) => {
                let o_v_op = other.raw_big_int();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::Float(v) => {
                let o_v_op = other.raw_float();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::BigFloat(v) => {
                let o_v_op = other.raw_big_float();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::Bool(v) => {
                let o_v_op = other.raw_bool();
                match o_v_op {
                    None => false,
                    Some(o_v) => v.eq(o_v),
                }
            }
            BmbpValue::NULL => other.is_null(),
            BmbpValue::Map(v) => {
                let op_v = other.raw_map();
                match op_v {
                    None => false,
                    Some(ov) => v.eq(ov),
                }
            }
            BmbpValue::Array(v) => {
                let op_v = other.raw_array();
                match op_v {
                    None => false,
                    Some(ov) => v.eq(ov),
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            BmbpValue::String(v) => {
                let o_v_op = other.raw_string();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::Int(v) => {
                let o_v_op = other.raw_int();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::BigInt(v) => {
                let o_v_op = other.raw_big_int();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::Float(v) => {
                let o_v_op = other.raw_float();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::BigFloat(v) => {
                let o_v_op = other.raw_big_float();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::Bool(v) => {
                let o_v_op = other.raw_bool();
                match o_v_op {
                    None => true,
                    Some(o_v) => v.ne(o_v),
                }
            }
            BmbpValue::NULL => !other.is_null(),
            BmbpValue::Map(v) => {
                let op_v = other.raw_map();
                match op_v {
                    None => true,
                    Some(ov) => v.ne(ov),
                }
            }
            BmbpValue::Array(v) => {
                let op_v = other.raw_array();
                match op_v {
                    None => true,
                    Some(ov) => v.ne(ov),
                }
            }
        }
    }
}

impl BmbpValue {
    pub fn v_type(&self) -> String {
        match self {
            BmbpValue::String(_) => "string".to_string(),
            BmbpValue::Int(_) => "int".to_string(),
            BmbpValue::BigInt(_) => "bigInt".to_string(),
            BmbpValue::Float(_) => "float".to_string(),
            BmbpValue::BigFloat(_) => "bigFloat".to_string(),
            BmbpValue::Bool(_) => "bool".to_string(),
            BmbpValue::Map(_) => "map".to_string(),
            BmbpValue::Array(_) => "array".to_string(),
            BmbpValue::NULL => "null".to_string(),
        }
    }
    pub fn is_string(&self) -> bool {
        self.v_type().eq(&"string".to_string())
    }
    pub fn is_int(&self) -> bool {
        self.v_type().eq(&"int".to_string())
    }
    pub fn is_big_int(&self) -> bool {
        self.v_type().eq(&"bigInt".to_string())
    }
    pub fn is_float(&self) -> bool {
        self.v_type().eq(&"float".to_string())
    }
    pub fn is_big_float(&self) -> bool {
        self.v_type().eq(&"bigFloat".to_string())
    }
    pub fn is_bool(&self) -> bool {
        self.v_type().eq(&"bool".to_string())
    }
    pub fn is_map(&self) -> bool {
        self.v_type().eq(&"map".to_string())
    }
    pub fn is_array(&self) -> bool {
        self.v_type().eq(&"array".to_string())
    }
    pub fn is_null(&self) -> bool {
        self.v_type().eq(&"null".to_string())
    }

    pub fn raw_string(&self) -> Option<&String> {
        match self {
            BmbpValue::String(v) => Some(v),
            _ => None,
        }
    }
    pub fn raw_int(&self) -> Option<&i32> {
        match self {
            BmbpValue::Int(v) => Some(v),
            _ => None,
        }
    }
    pub fn raw_big_int(&self) -> Option<&i128> {
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
    pub fn raw_map(&self) -> Option<&BmbpHashMap> {
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

impl ToString for BmbpValue {
    fn to_string(&self) -> String {
        match self {
            BmbpValue::String(v) => v.to_string(),
            BmbpValue::Int(v) => v.to_string(),
            BmbpValue::BigInt(v) => v.to_string(),
            BmbpValue::Float(v) => v.to_string(),
            BmbpValue::BigFloat(v) => v.to_string(),
            BmbpValue::Bool(v) => v.to_string(),
            BmbpValue::Map(_v) => "".to_string(),
            BmbpValue::Array(_v) => "".to_string(),
            BmbpValue::NULL => "".to_string(),
        }
    }
}

pub type BmbpVec = Vec<BmbpValue>;
pub type BmbpHashMap = HashMap<String, BmbpValue>;

#[cfg(test)]
mod tests {
    use crate::{BmbpHashMap, BmbpValue};

    #[test]
    fn test_bmbp_value() {
        let v = BmbpValue::from("xxxx");
        println!("{}", serde_json::to_string(&v).unwrap());

        let mut bp = BmbpHashMap::new();
        bp.insert("a".to_string(), BmbpValue::from("xxx"));

        println!("{}", serde_json::to_string(&bp).unwrap());
    }
}
