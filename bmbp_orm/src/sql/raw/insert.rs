use serde_json::Value;
use std::cell::{Ref, RefCell, RefMut};

use bmbp_types::BmbpResp;

use crate::sql::dml::{DMLFieldValue, DmlField};
use crate::sql::dql::Table;
use crate::sql::DynamicSQLParam;
use crate::InsertSQL;

pub struct RawInsertBuilder<'a> {
    insert: &'a InsertSQL,
    params: &'a DynamicSQLParam,
    raw_fields: RefCell<Vec<String>>,
    raw_values: RefCell<Vec<Value>>,
}

#[allow(dead_code)]
impl<'a> RawInsertBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_insert_vec = vec![];

        let raw_insert_into_table = self.build_insert_table(self.get_sql().get_table())?;
        self.build_insert_columns(self.get_sql().get_fields());
        raw_insert_vec.push(format!("INSERT INTO {}", raw_insert_into_table));

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
    fn build_insert_table(&self, tables: &[Table]) -> BmbpResp<String> {
        let mut raw_table_vec = vec![];
        for table in tables {
            let mut raw_table = table.table_name().clone();
            if !table.table_alias().is_empty() {
                raw_table = raw_table + " AS " + table.table_alias();
            }
            raw_table_vec.push(raw_table);
        }
        Ok(raw_table_vec.join(","))
    }
    fn build_insert_columns(&self, fields: &[DmlField]) {
        for field in fields {
            let column = field.get_field().clone();
            let value = field.get_value();
            match value {
                DMLFieldValue::SCRIPT(v) => {
                    let k_value = self.get_params().get_k_value(v.clone());
                    let raw_value = match k_value {
                        None => Value::Null,
                        Some(v) => v.clone(),
                    };
                    self.get_mut_raw_fields().push(column.clone());
                    self.get_mut_raw_values().push(raw_value);
                }
                DMLFieldValue::POSITION(p) => {
                    let p_value = self.get_params().get_p_value(p.clone());
                    let raw_value = match p_value {
                        None => Value::Null,
                        Some(v) => v.clone(),
                    };
                    self.get_mut_raw_fields().push(column.clone());
                    self.get_mut_raw_values().push(raw_value);
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
    pub fn new(insert: &'a InsertSQL, params: &'a DynamicSQLParam) -> Self {
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

    pub fn get_sql(&self) -> &InsertSQL {
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
