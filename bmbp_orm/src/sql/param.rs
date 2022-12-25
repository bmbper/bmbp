use std::collections::HashMap;

use serde_json::Value;

pub struct SQLParam {
    types: u8,
    p_params: Vec<Value>,
    m_params: HashMap<String, Value>,
}

impl SQLParam {
    pub fn new_p() -> Self {
        Self {
            types: 0,
            p_params: vec![],
            m_params: HashMap::new(),
        }
    }
    pub fn new_m() -> Self {
        Self {
            types: 1,
            p_params: vec![],
            m_params: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            types: 3,
            p_params: vec![],
            m_params: HashMap::new(),
        }
    }

    pub fn p_add(&mut self, param: Value) -> &mut Self {
        self.p_params.push(param);
        self
    }

    pub fn p_params(&self) -> &[Value] {
        self.p_params.as_slice()
    }

    pub fn p_params_mut(&mut self, param: Value) -> &mut [Value] {
        self.p_params.as_mut_slice()
    }

    pub fn m_add(&mut self, key: String, param: Value) -> &mut Self {
        self.m_params.insert(key, param);
        self
    }

    pub fn m_params(&self) -> &HashMap<String, Value> {
        &self.m_params
    }

    pub fn m_params_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.m_params
    }

    pub fn types(&self) -> u8 {
        self.types.clone()
    }
}

impl SQLParam {
    pub fn to_params(&self) -> &[Value] {
        self.p_params.as_slice()
    }
}
