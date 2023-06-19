use std::cell::RefCell;

use serde_json::Value;

use bmbp_app_common::BmbpResp;

use crate::sql::dql::{
    CompareField, CompareType, FilterField, FilterType, FilterValue, FuncCompareFieldInner,
    QueryComparefieldInner, QueryFilter,
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

        if field.get_cp().is_in() {
            let p_value = self.build_filter_in_field_value(field.get_value())?;
            if !p_value.is_empty() {
                self.raw_fields
                    .borrow_mut()
                    .push(format!(" {} {} ({})", column, cmp, p_value));
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

    fn build_filter_in_field_value(&self, v_type: &FilterValue) -> BmbpResp<String> {
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

        let raw_value: String = {
            match value {
                Value::Array(v_vec) => {
                    if v_vec.is_empty() {
                        "''".to_string()
                    } else {
                        let mut raw_vec = vec![];
                        for v in v_vec.as_slice() {
                            let mut v_s = self.value_to_string(v);
                            if v.is_string() {
                                v_s = format!("'{}'", v_s)
                            }
                            raw_vec.push(v_s);
                        }
                        raw_vec.join(",")
                    }
                }

                Value::String(v) => v.clone(),
                _ => "''".to_string(),
            }
        };
        Ok(raw_value)
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

    pub fn extend_values(&self, value: &[Value]) -> &Self {
        self.raw_values.borrow_mut().extend_from_slice(value);
        self
    }
}

pub struct RawDmlFilterBuilder<'a> {
    filter: &'a QueryFilter,
    params: &'a DynamicSQLParam,
    raw_values: &'a RefCell<Vec<Value>>,
    raw_field: RefCell<Vec<String>>,
}

impl<'a> RawDmlFilterBuilder<'a> {
    pub fn new(
        filter: &'a QueryFilter,
        params: &'a DynamicSQLParam,
        raw_values: &'a RefCell<Vec<Value>>,
    ) -> Self {
        let builder = RawDmlFilterBuilder {
            filter,
            params,
            raw_values,
            raw_field: RefCell::new(vec![]),
        };
        builder
    }
}

impl<'a> RawDmlFilterBuilder<'a> {
    pub fn get_filter(&self) -> &QueryFilter {
        self.filter
    }
    pub fn get_params(&self) -> &DynamicSQLParam {
        self.params
    }
    pub fn get_raw_values(&self) -> &RefCell<Vec<Value>> {
        &self.raw_values
    }
    pub fn get_raw_field(&self) -> &RefCell<Vec<String>> {
        &self.raw_field
    }
}

impl<'a> RawDmlFilterBuilder<'a> {
    pub fn build(&self) -> BmbpResp<String> {
        // 查询字段为空
        if self.get_filter().get_field_slice().is_empty() {
            return Ok("".to_string());
        }
        self.build_filter_field()?;
        let raw_fields_binding = self.get_raw_field().borrow();
        let raw_field_slice = raw_fields_binding.as_slice();
        FilterUtil::build_express(raw_field_slice, self.get_filter().get_filter_type())
    }

    pub fn build_filter_field(&self) -> BmbpResp<()> {
        let fields = self.get_filter().get_field_slice();
        let filter = RawFilterBuilder::new(fields, self.get_params());

        // 更新VALUE
        let mut raw_value_ref = self.get_raw_values().borrow_mut();
        filter.extend_values(raw_value_ref.as_slice());
        filter.build_filter()?;
        raw_value_ref.clear();
        raw_value_ref.extend_from_slice(filter.get_raw_values().as_slice());

        // 更新字段
        self.get_raw_field()
            .borrow_mut()
            .extend_from_slice(filter.get_raw_fields().as_slice());
        Ok(())
    }
}

pub struct FilterUtil();
impl FilterUtil {
    pub(crate) fn build_express<'a>(field: &'a [String], typ: &'a FilterType) -> BmbpResp<String> {
        match typ {
            FilterType::AND | FilterType::OR => {
                Ok(field.join(format!(" {} ", typ.to_string()).as_str()))
            }
            #[allow(unused)]
            FilterType::Express(express) => Ok("".to_string()),
        }
    }
}
