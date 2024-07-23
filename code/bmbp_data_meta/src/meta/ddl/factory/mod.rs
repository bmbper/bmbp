mod pg;

use crate::meta::ddl::factory::pg::PgDdlClient;
use crate::{BmbpDBType, BmbpRdbcTable};

pub trait DdlBuilder {
    fn build_create_table_sql(&self, table: &BmbpRdbcTable) -> Result<String, String>;
}

pub struct DdlBuilderFactory {}
impl DdlBuilderFactory {
    pub fn get_ddl_builder(db_type: &BmbpDBType) -> Option<Box<dyn DdlBuilder>> {
        match db_type {
            BmbpDBType::POSTGRESQL => Some(Box::new(PgDdlClient {})),
            _ => None,
        }
    }
}
