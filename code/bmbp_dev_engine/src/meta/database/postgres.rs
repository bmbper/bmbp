use crate::meta::meta::BmbpDbMetaEngine;
use bmbp_dev_dsl::{
    BmbpDBColumn, BmbpDBSchema, BmbpDBTable, BmbpDbConstraint, BmbpDbEvent, BmbpDbIndex,
    BmbpDbProcedure, BmbpDbTrigger, BmbpDbView,
};

pub struct PostgresMetaEngine;
impl BmbpDbMetaEngine for PostgresMetaEngine {
    fn query_schema_meta(&self) -> Option<BmbpDBSchema> {
        todo!()
    }

    fn query_table_meta(&self, table_name: String) -> Option<BmbpDBTable> {
        todo!()
    }

    fn query_column_meta(&self, table_name: String) -> Option<Vec<BmbpDBColumn>> {
        todo!()
    }

    fn query_index_meta(&self, table_name: String) -> Option<Vec<BmbpDbIndex>> {
        todo!()
    }

    fn query_constraint_meta(&self, table_name: String) -> Option<Vec<BmbpDbConstraint>> {
        todo!()
    }

    fn query_view_meta(&self, view_name: String) -> Option<BmbpDbView> {
        todo!()
    }

    fn query_trigger_meta(&self, trigger_name: String) -> Option<BmbpDbTrigger> {
        todo!()
    }

    fn query_procedure_meta(&self, procedure_name: String) -> Option<BmbpDbProcedure> {
        todo!()
    }

    fn query_event_meta(&self, event_name: String) -> Option<BmbpDbEvent> {
        todo!()
    }

    fn get_table_meta(&self, table_name: String) -> String {
        todo!()
    }
}
