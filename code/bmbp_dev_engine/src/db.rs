use bmbp_dev_dsl::*;

pub struct BmbpDdlTableEngine;

impl BmbpDdlTableEngine {
    pub fn create_table(&self, table: BmbpDBTable) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_table(&self, table: BmbpDBTable) -> String {
        return "".to_string();
    }
    pub fn alter_table(&self, table: BmbpDBTable) -> String {
        return "".to_string();
    }
    pub fn drop_table(&self, table: BmbpDBTable) -> String {
        return "".to_string();
    }
    pub fn drop_table_if_exists(&self, table: BmbpDBTable) -> String {
        return "".to_string();
    }
}
pub struct BmbpDdlColumnEngine;
impl BmbpDdlColumnEngine {
    pub fn add_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String {
        return "".to_string();
    }
    pub fn add_or_replace_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String {
        return "".to_string();
    }
    pub fn alter_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String {
        return "".to_string();
    }
    pub fn drop_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String {
        return "".to_string();
    }
    pub fn drop_column_if_exists(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String {
        return "".to_string();
    }
}

pub struct BmbpDdlIndexEngine;
impl BmbpDdlIndexEngine {
    pub fn create_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String {
        return "".to_string();
    }
    pub fn drop_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String {
        return "".to_string();
    }
    pub fn drop_index_if_exists(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String {
        return "".to_string();
    }
}

pub struct BmbpDdlConstraintEngine;
impl BmbpDdlConstraintEngine {
    pub fn add_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String {
        return "".to_string();
    }
    pub fn add_or_replace_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String {
        return "".to_string();
    }
    pub fn drop_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String {
        return "".to_string();
    }
    pub fn drop_constraint_if_exists(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String {
        return "".to_string();
    }
}

pub struct BmbpDbViewEngine;
impl BmbpDbViewEngine {
    pub fn create_view(&self, view: BmbpDbView) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_view(&self, view: BmbpDbView) -> String {
        return "".to_string();
    }
    pub fn drop_view(&self, view: BmbpDbView) -> String {
        return "".to_string();
    }
    pub fn drop_view_if_exists(&self, view: BmbpDbView) -> String {
        return "".to_string();
    }
}

pub struct BmbpDbTriggerEngine;
impl BmbpDbTriggerEngine {
    pub fn create_trigger(&self, trigger: BmbpDbTrigger) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_trigger(&self, trigger: BmbpDbTrigger) -> String {
        return "".to_string();
    }
    pub fn drop_trigger(&self, trigger: BmbpDbTrigger) -> String {
        return "".to_string();
    }
    pub fn drop_trigger_if_exists(&self, trigger: BmbpDbTrigger) -> String {
        return "".to_string();
    }
}

pub struct BmbpDbProcedureEngine;
impl BmbpDbProcedureEngine {
    pub fn create_procedure(&self, procedure: BmbpDbProcedure) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_procedure(&self, procedure: BmbpDbProcedure) -> String {
        return "".to_string();
    }
    pub fn drop_procedure(&self, procedure: BmbpDbProcedure) -> String {
        return "".to_string();
    }
    pub fn drop_procedure_if_exists(&self, procedure: BmbpDbProcedure) -> String {
        return "".to_string();
    }
}


pub struct BmbpDbEventEngine;
impl BmbpDbEventEngine {
    pub fn create_event(&self, event: BmbpDbEvent) -> String {
        return "".to_string();
    }
    pub fn create_or_replace_event(&self, event: BmbpDbEvent) -> String {
        return "".to_string();
    }
    pub fn drop_event(&self, event: BmbpDbEvent) -> String {
        return "".to_string();
    }
    pub fn drop_event_if_exists(&self, event: BmbpDbEvent) -> String {
        return "".to_string();
    }
}