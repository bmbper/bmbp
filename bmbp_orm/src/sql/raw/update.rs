use std::cell::RefCell;

use serde_json::Value;

use bmbp_types::BmbpResp;

use crate::sql::DynamicSQLParam;
use crate::UpdateSQL;

#[allow(dead_code)]
pub struct RawUpdateBuilder<'a> {
    update: &'a UpdateSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

impl<'a> RawUpdateBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}

impl<'a> RawUpdateBuilder<'a> {
    pub fn new(update: &'a UpdateSQL, params: &'a DynamicSQLParam) -> Self {
        RawUpdateBuilder {
            update,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }
}
