use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub struct DynamicSQLParam {
    k_params: HashMap<String, Value>,
    p_params: Vec<Value>,
}

impl DynamicSQLParam {
    pub fn new() -> DynamicSQLParam {
        DynamicSQLParam {
            k_params: HashMap::new(),
            p_params: vec![],
        }
    }
    pub fn set_k_params(&mut self, params: HashMap<String, Value>) -> &mut Self {
        self.k_params = params;
        self
    }
    pub fn get_k_params(&self) -> &HashMap<String, Value> {
        &self.k_params
    }
    pub fn set_p_params(&mut self, params: Vec<Value>) -> &mut Self {
        self.p_params = params;
        self
    }
    pub fn get_p_params(&self) -> &[Value] {
        self.p_params.as_slice()
    }
    pub fn extend_k_params(&mut self, params: HashMap<String, Value>) -> &mut Self {
        self.k_params.extend(params);
        self
    }

    pub fn extend_p_params(&mut self, params: Vec<Value>) -> &mut Self {
        self.p_params.extend_from_slice(params.as_slice());
        self
    }

    pub fn add_k_param(&mut self, key: String, param: Value) -> &mut Self {
        self.k_params.insert(key, param);
        self
    }

    pub fn add_p_param(&mut self, param: Value) -> &mut Self {
        self.p_params.push(param);
        self
    }

    pub fn get_k_value(&self, key: String) -> Option<&Value> {
        self.get_k_params().get(key.as_str())
    }
    pub fn get_p_value(&self, position: usize) -> Option<&Value> {
        self.get_p_params().get(position)
    }
}
