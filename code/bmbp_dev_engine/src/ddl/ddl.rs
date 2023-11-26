use bmbp_dev_dsl::*;

pub trait BmbpDbDdlSchemaEngine {
    fn create_schema(&self, schema: BmbpDBSchema) -> String;
    fn create_or_replace_schema(&self, schema: BmbpDBSchema) -> String;
    fn drop_schema(&self, schema: BmbpDBSchema) -> String;
}
pub trait BmbpDbDdlTableEngine {
    fn create_table(&self, table: BmbpDBTable) -> String;
    fn create_or_replace_table(&self, table: BmbpDBTable) -> String;
    fn alter_table(&self, table: BmbpDBTable) -> String;
    fn drop_table(&self, table: BmbpDBTable) -> String;
}
pub trait BmbpDbDdlColumnEngine {
    fn add_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String;
    fn add_or_replace_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String;
    fn alter_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String;
    fn drop_column(&self, table: BmbpDBTable, column: BmbpDBColumn) -> String;
}
pub trait BmbpDbDdlIndexEngine {
    fn create_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String;
    fn create_or_replace_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String;
    fn drop_index(&self, table: BmbpDBTable, index: BmbpDbIndex) -> String;
}
pub trait BmbpDbDdlConstraintEngine {
    fn add_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String;
    fn add_or_replace_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String;
    fn drop_constraint(&self, table: BmbpDBTable, constraint: BmbpDbConstraint) -> String;
}
pub trait BmbpDbDdlViewEngine {
    fn create_view(&self, view: BmbpDbView) -> String;
    fn create_or_replace_view(&self, view: BmbpDbView) -> String;
    fn drop_view(&self, view: BmbpDbView) -> String;
}
pub trait BmbpDbDdlTriggerEngine {
    fn create_trigger(&self, trigger: BmbpDbTrigger) -> String;
    fn create_or_replace_trigger(&self, trigger: BmbpDbTrigger) -> String;
    fn drop_trigger(&self, trigger: BmbpDbTrigger) -> String;
}
pub trait BmbpDbDdlProcedureEngine {
    fn create_procedure(&self, procedure: BmbpDbProcedure) -> String;
    fn create_or_replace_procedure(&self, procedure: BmbpDbProcedure) -> String;
    fn drop_procedure(&self, procedure: BmbpDbProcedure) -> String;
}
pub trait BmbpDbDdlEventEngine {
    fn create_event(&self, event: BmbpDbEvent) -> String;
    fn create_or_replace_event(&self, event: BmbpDbEvent) -> String;
    fn drop_event(&self, event: BmbpDbEvent) -> String;
}