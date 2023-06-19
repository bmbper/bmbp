use serde_json::Value;

use bmbp_app_common::BmbpResp;

use crate::sql::{DdlSQL, DynamicSQLParam};

#[allow(dead_code)]
pub struct RawDDLBuilder<'a> {
    ddl: &'a DdlSQL,
    params: &'a DynamicSQLParam,
}

#[allow(dead_code)]
impl<'a> RawDDLBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}

#[allow(dead_code)]
impl<'a> RawDDLBuilder<'a> {
    pub fn new(ddl: &'a DdlSQL, params: &'a DynamicSQLParam) -> Self {
        RawDDLBuilder { ddl, params }
    }
}
