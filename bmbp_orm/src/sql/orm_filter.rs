use crate::sql::dql::FilterField;
use crate::sql::DynamicSQLParam;
use bmbp_types::BmbpResp;
use serde_json::Value;

pub struct FilterBuilder<'a> {
    fields: &'a [FilterField],
    params: &'a DynamicSQLParam,
}

impl<'a> FilterBuilder<'a> {
    pub(crate) fn build_filter(&self) -> BmbpResp<(Vec<String>, Vec<Value>)> {
        Ok((vec![], vec![]))
    }
}

impl<'a> FilterBuilder<'a> {
    pub fn new(fields: &'a [FilterField], params: &'a DynamicSQLParam) -> Self {
        FilterBuilder { fields, params }
    }
}
