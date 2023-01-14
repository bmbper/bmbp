use serde_json::Value;

use bmbp_types::BmbpResp;

use crate::sql::DynamicSQLParam;
use crate::DeleteSQL;

#[allow(dead_code)]
pub struct RawDeleteBuilder<'a> {
    delete: &'a DeleteSQL,
    params: &'a DynamicSQLParam,
}

#[allow(dead_code)]
impl<'a> RawDeleteBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}
#[allow(dead_code)]
impl<'a> RawDeleteBuilder<'a> {
    pub fn new(delete: &'a DeleteSQL, params: &'a DynamicSQLParam) -> Self {
        RawDeleteBuilder { delete, params }
    }
}
