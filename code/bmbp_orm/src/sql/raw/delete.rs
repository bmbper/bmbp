use serde_json::Value;
use std::cell::{Ref, RefCell, RefMut};

use bmbp_types::BmbpResp;

use crate::sql::raw::filter::RawDmlFilterBuilder;
use crate::sql::raw::table::TableBuilder;
use crate::sql::DynamicSQLParam;
use crate::BmbpDeleteSQL;

#[allow(dead_code)]
pub struct RawDeleteBuilder<'a> {
    delete: &'a BmbpDeleteSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

#[allow(dead_code)]
impl<'a> RawDeleteBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_delete_vec = vec![];

        let delete_from = self.build_target_table()?;
        raw_delete_vec.push(format!(" DELETE FROM {} ", delete_from));

        let delete_filter = self.build_filter()?;
        if !delete_filter.is_empty() {
            raw_delete_vec.push(format!("WHERE {}", delete_filter));
        }
        Ok((raw_delete_vec.join(" "), self.get_raw_values()))
    }
    fn build_target_table(&self) -> BmbpResp<String> {
        let table_slice = self.get_sql().get_table_slice();
        TableBuilder::delete(table_slice).build()
    }
    fn build_filter(&self) -> BmbpResp<String> {
        if let Some(filter) = self.get_sql().get_filter() {
            Ok(
                RawDmlFilterBuilder::new(filter, self.get_params(), self.get_raw_values_ref())
                    .build()?,
            )
        } else {
            Ok("".to_string())
        }
    }
}
#[allow(dead_code)]
impl<'a> RawDeleteBuilder<'a> {
    pub fn new(delete: &'a BmbpDeleteSQL, params: &'a DynamicSQLParam) -> Self {
        RawDeleteBuilder {
            delete,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }
    pub fn get_params(&self) -> &DynamicSQLParam {
        self.params
    }

    pub fn get_sql(&self) -> &BmbpDeleteSQL {
        self.delete
    }

    pub fn get_raw_fields(&self) -> Ref<'_, Vec<String>> {
        self.raw_fields.borrow()
    }
    pub fn get_raw_values(&self) -> Vec<Value> {
        self.raw_values.borrow().clone()
    }
    pub fn get_mut_raw_fields(&self) -> RefMut<'_, Vec<String>> {
        self.raw_fields.borrow_mut()
    }
    pub fn get_mut_raw_values(&self) -> RefMut<'_, Vec<Value>> {
        self.raw_values.borrow_mut()
    }
    pub fn get_raw_values_ref(&self) -> &RefCell<Vec<Value>> {
        &self.raw_values
    }
}
