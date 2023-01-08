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
pub enum DMLFieldValueType {
    SCRIPT,
    VALUE,
}

#[derive(Clone)]
pub struct DmlField {
    field: String,
    value: Value,
}
