use bmbp_types::BmbpResp;
use serde_json::Value;
use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::sql::dql::{
    CompareField, FilterField, FilterValue, FuncCompareFieldInner, QueryCompareFiledInner,
};
use crate::sql::DynamicSQLParam;

pub struct RawFilterBuilder<'a> {
    fields: &'a [FilterField],
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

impl<'a> RawFilterBuilder<'a> {
    pub(crate) fn build_filter(&self) -> BmbpResp<(Vec<String>, Vec<Value>)> {
        let (mut raw_field_vec, mut raw_params_vec) = (vec![], vec![]);
        let filter_fields = self.get_fields();
        for filter_field in filter_fields {
            self.build_filter_field(filter_field)?;
        }
        Ok((raw_field_vec, raw_params_vec))
    }
    fn build_filter_field(&self, field: &FilterField) -> BmbpResp<()> {
        let column = self.build_filter_column(field.get_column())?;
        let cmp = field.get_cp().to_string();
        if field.get_cp().is_null() {
            self.raw_fields
                .borrow_mut()
                .push(format!(" {} {} ", column, cmp));
            return Ok(());
        }

        if field.get_cp().is_simple() {
            let p_value = self.build_filter_simple_field_value(field.get_value())?;
            self.raw_fields
                .borrow_mut()
                .push(format!(" {} {} {}", column, cmp, p_value));
            return Ok(());
        }

        Ok(())
    }

    fn build_filter_simple_field_value(&self, v_type: &FilterValue) -> BmbpResp<String> {
        match v_type {
            FilterValue::SCRIPT(field_key) => {
                let k_value = self.get_params().get_k_value(field_key.clone());
                let raw_value = {
                    if let Some(rv) = k_value {
                        rv.clone()
                    } else {
                        Value::Null
                    }
                };
                let p = self.raw_values.borrow().len();
                self.raw_values.borrow_mut().push(raw_value);
                Ok(format!("${}", p))
            }
            FilterValue::POSITION(position) => {
                let p_value = self.get_params().get_p_value(position.clone());
                let raw_value = {
                    if let Some(rv) = p_value {
                        rv.clone()
                    } else {
                        Value::Null
                    }
                };
                let p = self.raw_values.borrow().len();
                self.raw_values.borrow_mut().push(raw_value);
                Ok(format!("${}", p))
            }
            FilterValue::VALUE(v) => {
                let p = self.raw_values.borrow().len();
                self.raw_values.borrow_mut().push(v.clone());
                Ok(format!("${}", p))
            }
        }
    }

    fn build_filter_column(&self, field: &CompareField) -> BmbpResp<String> {
        match field {
            CompareField::Column(column) => Ok(column.clone()),
            CompareField::Func(field) => self.build_filter_func(field),
            CompareField::Query(field) => self.build_filter_query(field),
        }
    }
    fn build_filter_func(&self, _field: &FuncCompareFieldInner) -> BmbpResp<String> {
        Ok("".to_string())
    }
    fn build_filter_query(&self, _field: &QueryCompareFiledInner) -> BmbpResp<String> {
        Ok("".to_string())
    }
}

impl<'a> RawFilterBuilder<'a> {
    pub fn new(fields: &'a [FilterField], params: &'a DynamicSQLParam) -> Self {
        RawFilterBuilder {
            fields,
            params,
            raw_fields: RefCell::new(vec![]),
            raw_values: RefCell::new(vec![]),
        }
    }
    pub fn get_fields(&self) -> &[FilterField] {
        self.fields
    }

    pub fn get_params(&self) -> &DynamicSQLParam {
        self.params
    }

    pub fn get_raw_fields(&self) -> Vec<String> {
        self.raw_fields.borrow().clone()
    }

    pub fn get_raw_values(&self) -> Vec<Value> {
        self.raw_values.borrow().clone()
    }

    pub fn add_raw_field(&mut self, field: String) -> &mut Self {
        self.raw_fields.borrow_mut().push(field);
        self
    }
    pub fn add_raw_value(&mut self, value: Value) -> &mut Self {
        self.raw_values.borrow_mut().push(value);
        self
    }
}
