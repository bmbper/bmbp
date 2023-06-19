use serde_json::Value;
use std::cell::{Ref, RefCell, RefMut};

use bmbp_app_common::BmbpResp;

use crate::sql::dml::{DMLFieldValue, DmlField};
use crate::sql::raw::table::TableBuilder;
use crate::sql::DynamicSQLParam;
use crate::BmbpInsertSQL;

pub struct RawInsertBuilder<'a> {
    insert: &'a BmbpInsertSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

#[allow(dead_code)]
impl<'a> RawInsertBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_insert_vec = vec![];

        let raw_insert_into_table = self.build_insert_table()?;
        raw_insert_vec.push(format!("INSERT INTO {}", raw_insert_into_table));

        self.build_insert_columns(self.get_sql().get_fields());
        let mut raw_insert_columns = vec![];
        let mut raw_insert_values = vec![];
        for (index, raw_field) in self.get_raw_fields().as_slice().iter().enumerate() {
            raw_insert_columns.push(raw_field.clone());
            raw_insert_values.push(format!("${}", index + 1));
        }
        let raw_insert_column_sql = format!("({})", raw_insert_columns.join(","));
        raw_insert_vec.push(raw_insert_column_sql);
        let raw_insert_value_sql = format!("VALUES({})", raw_insert_values.join(","));
        raw_insert_vec.push(raw_insert_value_sql);

        Ok((raw_insert_vec.join(" "), self.get_raw_values()))
    }
    fn build_insert_table(&self) -> BmbpResp<String> {
        TableBuilder::insert(self.get_sql().get_table()).build()
    }
    fn build_insert_columns(&self, fields: &[DmlField]) {
        for field in fields {
            let column = field.get_field().clone();
            let value = field.get_value();
            match value {
                DMLFieldValue::SCRIPT(v) => {
                    let k_value = self.get_params().get_k_value(v.clone());
                    if let Some(raw_value) = k_value {
                        self.get_mut_raw_fields().push(column.clone());
                        self.get_mut_raw_values().push(raw_value.clone());
                    }
                }
                DMLFieldValue::POSITION(p) => {
                    let p_value = self.get_params().get_p_value(p.clone());
                    if let Some(raw_value) = p_value {
                        self.get_mut_raw_fields().push(column.clone());
                        self.get_mut_raw_values().push(raw_value.clone());
                    }
                }
                DMLFieldValue::VALUE(v) => {
                    self.get_mut_raw_fields().push(column.clone());
                    self.get_mut_raw_values().push(v.clone());
                }
            }
        }
    }
}

impl<'a> RawInsertBuilder<'a> {
    pub fn new(insert: &'a BmbpInsertSQL, params: &'a DynamicSQLParam) -> Self {
        RawInsertBuilder {
            insert,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }

    pub fn get_params(&self) -> &DynamicSQLParam {
        self.params
    }

    pub fn get_sql(&self) -> &BmbpInsertSQL {
        self.insert
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
}
