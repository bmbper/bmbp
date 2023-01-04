use std::collections::HashMap;

use serde_json::Value;

pub struct DynamicSQLParam {
    params: HashMap<String, Value>,
    param_keys: Vec<String>,
}

impl DynamicSQLParam {
    pub fn new() -> DynamicSQLParam {
        DynamicSQLParam {
            params: HashMap::new(),
            param_keys: vec![],
        }
    }
    pub fn to_sql_params(&self) -> Vec<Value> {
        let mut params_vec = vec![];
        for key in &self.param_keys {
            if let Some(value) = self.params.get(key.as_str()) {
                params_vec.push(value.clone());
            } else {
                params_vec.push(Value::Null)
            }
        }
        params_vec
    }
    pub fn set_params(&mut self, params: HashMap<String, Value>) -> &mut Self {
        self.params = params;
        self
    }

    pub fn set_keys(&mut self, keys: Vec<String>) -> &mut Self {
        self.param_keys = keys;
        self
    }

    pub fn add_keys(&mut self, key: String) -> &mut Self {
        self.param_keys.push(key);
        self
    }

    pub fn add_param(&mut self, key: String, value: Value) -> &mut Self {
        self.params.insert(key, value);
        self
    }
}
