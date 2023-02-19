use std::cell::{Ref, RefCell, RefMut};

use serde_json::Value;

use bmbp_types::{BmbpError, BmbpResp};

use crate::sql::dml::{DMLFieldValue, DmlField};
use crate::sql::raw::filter::RawDmlFilterBuilder;
use crate::sql::raw::table::TableBuilder;
use crate::sql::DynamicSQLParam;
use crate::BmbpUpdateSQL;

#[allow(dead_code)]
pub struct RawUpdateBuilder<'a> {
    update: &'a BmbpUpdateSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

impl<'a> RawUpdateBuilder<'a> {
    pub fn new(update: &'a BmbpUpdateSQL, params: &'a DynamicSQLParam) -> Self {
        RawUpdateBuilder {
            update,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }
}

#[allow(dead_code)]
impl<'a> RawUpdateBuilder<'a> {
    pub fn get_params(&self) -> &DynamicSQLParam {
        self.params
    }

    pub fn get_sql(&self) -> &BmbpUpdateSQL {
        self.update
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
#[allow(dead_code)]
impl<'a> RawUpdateBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_update_vec = vec![];
        let raw_update_table = self.build_update_table()?;
        raw_update_vec.push(format!(" UPDATE {} ", raw_update_table));
        let raw_set_fields = self.build_update_fields()?;
        raw_update_vec.push(format!(" SET {} ", raw_set_fields));

        let raw_filter = self.build_update_filter()?;
        if !raw_filter.is_empty() {
            raw_update_vec.push(format!(" WHERE {} ", raw_filter));
        }
        Ok((raw_update_vec.join(" "), self.get_raw_values()))
    }
    fn build_update_table(&self) -> BmbpResp<String> {
        let table_slice = self.get_sql().get_table_slice();

        TableBuilder::update(table_slice).build()
    }
    fn build_update_fields(&self) -> BmbpResp<String> {
        let fields = self.get_sql().get_fields();
        let mut raw_field_vec = vec![];
        for field in fields {
            let raw_field = self.build_update_field(field)?;
            raw_field_vec.push(raw_field);
        }
        if !raw_field_vec.is_empty() {
            Ok(raw_field_vec.join(","))
        } else {
            Err(BmbpError::dyn_sql("请指定要更新的字段".to_string()))
        }
    }
    fn build_update_field(&self, field: &DmlField) -> BmbpResp<String> {
        let column = field.get_field();
        if column.is_empty() {
            return Err(BmbpError::dyn_sql("请指定要更新的字段".to_string()));
        }
        let value = match field.get_value() {
            DMLFieldValue::SCRIPT(v) => {
                let k_value = self.get_params().get_k_value(v.clone());
                let raw_value = match k_value {
                    None => Value::Null,
                    Some(v) => v.clone(),
                };
                self.get_mut_raw_values().push(raw_value);
                self.get_raw_values().len()
            }
            DMLFieldValue::POSITION(p) => {
                let p_value = self.get_params().get_p_value(p.clone());
                let raw_value = match p_value {
                    None => Value::Null,
                    Some(v) => v.clone(),
                };
                self.get_mut_raw_values().push(raw_value);
                self.get_raw_values().len()
            }
            DMLFieldValue::VALUE(v) => {
                self.get_mut_raw_values().push(v.clone());
                self.get_raw_values().len()
            }
        };
        Ok(format!("{} = ${}", column, value))
    }
    fn build_update_filter(&self) -> BmbpResp<String> {
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
