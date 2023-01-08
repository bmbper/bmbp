use crate::sql::DynamicSQLParam;
use crate::InsertSQL;
use bmbp_types::BmbpResp;
use serde_json::Value;
use std::cell::RefCell;

pub struct RawInsertBuilder<'a> {
    insert: &'a InsertSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

impl<'a> RawInsertBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}

impl<'a> RawInsertBuilder<'a> {
    pub fn new(insert: &'a InsertSQL, params: &'a DynamicSQLParam) -> Self {
        RawInsertBuilder {
            insert,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }
}
