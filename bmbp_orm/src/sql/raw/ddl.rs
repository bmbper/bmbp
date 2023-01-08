use crate::sql::{DdlSQL, DynamicSQLParam};
use crate::{DeleteSQL, InsertSQL, UpdateSQL};
use bmbp_types::BmbpResp;
use serde_json::Value;

pub struct RawDDLBuilder<'a> {
    ddl: &'a DdlSQL,
    params: &'a DynamicSQLParam,
}

impl<'a> RawDDLBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}

impl<'a> RawDDLBuilder<'a> {
    pub fn new(ddl: &'a DdlSQL, params: &'a DynamicSQLParam) -> Self {
        RawDDLBuilder { ddl, params }
    }
}
