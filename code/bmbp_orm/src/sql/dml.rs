use serde_json::Value;

use bmbp_app_utils::{camel_to_snake_upper, snake_to_camel};

use super::dql::{QueryFilter, Table};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BmbpInsertSQL {
    field: Vec<DmlField>,
    table: Vec<Table>,
}
#[allow(dead_code)]
impl BmbpInsertSQL {
    pub fn new() -> BmbpInsertSQL {
        BmbpInsertSQL {
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

    pub fn target_table(&mut self, table: String) -> &mut Self {
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BmbpUpdateSQL {
    field: Vec<DmlField>,
    table: Vec<Table>,
    filter: Option<QueryFilter>,
}
#[allow(dead_code)]
impl BmbpUpdateSQL {
    pub fn new() -> BmbpUpdateSQL {
        BmbpUpdateSQL {
            field: vec![],
            table: vec![],
            filter: None,
        }
    }
}

impl BmbpUpdateSQL {
    pub fn get_mut_filter(&mut self) -> &mut QueryFilter {
        if self.filter.is_none() {
            self.filter = Some(QueryFilter::new())
        }
        self.filter.as_mut().unwrap()
    }
    pub fn get_table_slice(&self) -> &[Table] {
        self.table.as_slice()
    }
    pub fn get_fields(&self) -> &[DmlField] {
        self.field.as_slice()
    }
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter.as_ref()
    }
}

impl BmbpUpdateSQL {
    pub fn target_table(&mut self, table: String) -> &mut Self {
        self.table.push(Table::new(table));
        self
    }
    pub fn set_s_f(&mut self, field: String) -> &mut Self {
        self.set_s_c_as(camel_to_snake_upper(field.clone()), field)
    }
    pub fn set_s_c_as(&mut self, column: String, field: String) -> &mut Self {
        self.field.push(DmlField::s_c_v(column, field));
        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BmbpDeleteSQL {
    table: Vec<Table>,
    filter: Option<QueryFilter>,
}

#[allow(dead_code)]
impl BmbpDeleteSQL {
    pub fn new() -> BmbpDeleteSQL {
        BmbpDeleteSQL {
            table: vec![],
            filter: None,
        }
    }
    pub fn get_mut_filter(&mut self) -> &mut QueryFilter {
        if self.filter.is_none() {
            self.filter = Some(QueryFilter::new())
        }
        self.filter.as_mut().unwrap()
    }
    pub fn get_table_slice(&self) -> &[Table] {
        self.table.as_slice()
    }
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter.as_ref()
    }
}

impl BmbpDeleteSQL {
    pub fn target_table(&mut self, table: String) -> &mut Self {
        self.table.push(Table::new(table));
        self
    }
}

#[derive(Clone, Debug)]
pub enum DMLFieldValue {
    SCRIPT(String),
    POSITION(usize),
    VALUE(Value),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DmlField {
    column: String,
    value: DMLFieldValue,
}

#[allow(dead_code)]
impl DmlField {
    pub fn s_c_v(column: String, value: String) -> Self {
        DmlField {
            column,
            value: DMLFieldValue::SCRIPT(value),
        }
    }

    pub fn p_c_v(column: String, value: usize) -> Self {
        DmlField {
            column,
            value: DMLFieldValue::POSITION(value),
        }
    }

    pub fn r_c_v(column: String, value: Value) -> Self {
        DmlField {
            column,
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
