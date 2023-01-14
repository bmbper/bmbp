use std::cell::RefCell;

use serde_json::Value;

use bmbp_types::BmbpResp;

use crate::sql::dql::{
    CompareField, CompareType, FilterField, FilterValue, FuncCompareFieldInner,
    QueryComparefieldInner,
};
use crate::sql::DynamicSQLParam;

#[allow(dead_code)]
pub struct RawFilterBuilder<'a> {
    fields: &'a [FilterField],
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}
#[allow(dead_code)]
impl<'a> RawFilterBuilder<'a> {
    pub(crate) fn build_filter(&self) -> BmbpResp<()> {
        let filter_fields = self.get_fields();
        for filter_field in filter_fields {
            self.build_filter_field(filter_field)?;
        }
        Ok(())
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
            let p_value = self.build_filter_field_value(field.get_value())?;
            if !p_value.is_empty() {
                self.raw_fields
                    .borrow_mut()
                    .push(format!(" {} {} {}", column, cmp, p_value));
            }
            return Ok(());
        }

        if field.get_cp().is_like() {
            let p_value = self.build_filter_like_field_value(field.get_cp(), field.get_value())?;
            if !p_value.is_empty() {
                self.raw_fields
                    .borrow_mut()
                    .push(format!(" {} {} {}", column, cmp, p_value));
            }

            return Ok(());
        }

        Ok(())
    }

    fn build_filter_field_value(&self, v_type: &FilterValue) -> BmbpResp<String> {
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
                let p = self.raw_values.borrow().len() + 1;
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
            #[allow(unused)]
            FilterValue::Filter(filter) => Ok("".to_string()),
            #[allow(unused)]
            FilterValue::Query(query) => Ok("".to_string()),
        }
    }

    fn build_filter_like_field_value(
        &self,
        cmp: &CompareType,
        v_type: &FilterValue,
    ) -> BmbpResp<String> {
        let value = match v_type {
            FilterValue::SCRIPT(field_key) => {
                let k_value = self.get_params().get_k_value(field_key.clone());
                let raw_value = {
                    if let Some(rv) = k_value {
                        rv.clone()
                    } else {
                        Value::Null
                    }
                };
                raw_value
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
                raw_value
            }
            FilterValue::VALUE(v) => v.clone(),
            #[allow(unused)]
            FilterValue::Filter(filter) => Value::Null,
            #[allow(unused)]
            FilterValue::Query(query) => Value::Null,
        };

        let value_string = self.value_to_string(&value);

        let raw_value = match cmp {
            CompareType::LK | CompareType::NLK => Value::String(format!("%{}%", value_string)),
            CompareType::RLK | CompareType::NRLK => Value::String(format!("%{}", value_string)),
            CompareType::LLK | CompareType::NLLK => Value::String(format!("{}%", value_string)),
            _ => value,
        };

        let p = self.raw_values.borrow().len() + 1;
        self.raw_values.borrow_mut().push(raw_value);
        Ok(format!("${}", p))
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
    fn build_filter_query(&self, _field: &QueryComparefieldInner) -> BmbpResp<String> {
        Ok("".to_string())
    }

    fn value_to_string(&self, v: &Value) -> String {
        match v {
            Value::Null => "".to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.to_string(),
            Value::Array(v) => serde_json::to_string(&v).unwrap(),
            Value::Object(v) => serde_json::to_string(&v).unwrap(),
        }
    }
}

#[allow(dead_code)]
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
