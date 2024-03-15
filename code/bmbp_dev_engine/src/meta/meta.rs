use bmbp_dev_dsl::{
    BmbpDBColumn, BmbpDBSchema, BmbpDBTable, BmbpDbConstraint, BmbpDbEvent, BmbpDbIndex,
    BmbpDbProcedure, BmbpDbTrigger, BmbpDbView,
};
use std::format;

// /code/bmbp_dev_engine/src/meta.rs
pub trait BmbpDbMetaEngine {
    fn query_schema_meta(&self) -> Option<BmbpDBSchema>;
    fn query_table_meta(&self, table_name: String) -> Option<BmbpDBTable>;
    fn query_column_meta(&self, table_name: String) -> Option<Vec<BmbpDBColumn>>;
    fn query_index_meta(&self, table_name: String) -> Option<Vec<BmbpDbIndex>>;
    fn query_constraint_meta(&self, table_name: String) -> Option<Vec<BmbpDbConstraint>>;
    fn query_view_meta(&self, view_name: String) -> Option<BmbpDbView>;
    fn query_trigger_meta(&self, trigger_name: String) -> Option<BmbpDbTrigger>;
    fn query_procedure_meta(&self, procedure_name: String) -> Option<BmbpDbProcedure>;
    fn query_event_meta(&self, event_name: String) -> Option<BmbpDbEvent>;
    fn get_table_meta(&self, table_name: String) -> String;
}
