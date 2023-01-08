use crate::sql::DynamicSQLParam;
use crate::{DeleteSQL, InsertSQL, UpdateSQL};
use bmbp_types::BmbpResp;
use serde_json::Value;

pub struct RawDeleteBuilder<'a> {
    delete: &'a DeleteSQL,
    params: &'a DynamicSQLParam,
}

impl<'a> RawDeleteBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }
}

impl<'a> RawDeleteBuilder<'a> {
    pub fn new(delete: &'a DeleteSQL, params: &'a DynamicSQLParam) -> Self {
        RawDeleteBuilder { delete, params }
    }
}
