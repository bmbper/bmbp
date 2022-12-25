use serde_json::Value;

use super::dql::{QueryFilter, Table};

pub struct InsertSQL {
    field: Vec<InsOrUpdField<Value>>,
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

pub struct UpdateSQL {
    field: Vec<InsOrUpdField<Value>>,
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

pub struct InsOrUpdField<Value> {
    field: String,
    value: Option<Value>,
}
