use bmbp_util::{camel_to_snake_upper, snake_to_camel};
use serde_json::Value;

use super::dql::{QueryFilter, Table};

#[derive(Clone)]
pub struct InsertSQL {
    field: Vec<DmlField>,
    table: Vec<Table>,
}

impl InsertSQL {
    pub fn new() -> InsertSQL {
        InsertSQL {
            field: vec![],
            table: vec![],
        }
    }

    pub fn get_table(&self) -> &[Table] {
        self.table.as_slice()
    }

    pub fn get_fields(&self) -> &[DmlField] {
        self.field.as_slice()
    }

    pub fn insert_into(&mut self, table: String) -> &mut Self {
        self.table.push(Table::new(table));
        self
    }

    pub fn f_insert(&mut self, field: String) -> &mut Self {
        let column = camel_to_snake_upper(field.clone());
        self.field.push(DmlField::s_c_v(column, field.clone()));
        self
    }

    pub fn c_insert(&mut self, column: String) -> &mut Self {
        let value = snake_to_camel(column.clone());
        self.field.push(DmlField::s_c_v(column, value));
        self
    }

    pub fn f_insert_as(&mut self, field: String, alias: String) -> &mut Self {
        let column = camel_to_snake_upper(field.clone());
        self.field.push(DmlField::s_c_v(column, alias));
        self
    }

    pub fn c_insert_as(&mut self, column: String, alias: String) -> &mut Self {
        self.field.push(DmlField::s_c_v(column, alias));
        self
    }

    pub fn f_insert_v(&mut self, field: String, value: Value) -> &mut Self {
        let column = camel_to_snake_upper(field.clone());
        self.field.push(DmlField::r_c_v(column, value));
        self
    }

    pub fn c_insert_v(&mut self, column: String, value: Value) -> &mut Self {
        self.field.push(DmlField::r_c_v(column, value));
        self
    }
}

#[derive(Clone)]
pub struct UpdateSQL {
    field: Vec<DmlField>,
    table: Vec<Table>,
    filter: Option<QueryFilter>,
}

impl UpdateSQL {
    pub fn new() -> UpdateSQL {
        UpdateSQL {
            field: vec![],
            table: vec![],
            filter: None,
        }
    }
}
#[derive(Clone)]
pub struct DeleteSQL {
    table: Vec<Table>,
    filter: Option<QueryFilter>,
}

impl DeleteSQL {
    pub fn new() -> DeleteSQL {
        DeleteSQL {
            table: vec![],
            filter: None,
        }
    }
}

#[derive(Clone)]
pub enum DMLFieldValue {
    SCRIPT(String),
    POSITION(usize),
    VALUE(Value),
}

#[derive(Clone)]
pub struct DmlField {
    column: String,
    value: DMLFieldValue,
}

impl DmlField {
    pub fn s_c_v(column: String, value: String) -> Self {
        DmlField {
            column,
            value: DMLFieldValue::SCRIPT(value),
        }
    }

    pub fn p_c_v(column: String, value: usize) -> Self {
        DmlField {
            column: column,
            value: DMLFieldValue::POSITION(value),
        }
    }

    pub fn r_c_v(column: String, value: Value) -> Self {
        DmlField {
            column: column,
            value: DMLFieldValue::VALUE(value),
        }
    }

    pub fn get_field(&self) -> &String {
        &self.column
    }
    pub fn get_value(&self) -> &DMLFieldValue {
        &self.value
    }
}
