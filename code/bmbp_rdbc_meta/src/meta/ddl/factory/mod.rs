mod pg;

use crate::meta::ddl::factory::pg::PgDdlClient;
use crate::{DataBaseType, RdbcTableVo};

pub trait DdlBuilder {
    fn build_create_table_sql(&self, table: &RdbcTableVo) -> Result<String, String>;
}

pub struct DdlBuilderFactory {}
impl DdlBuilderFactory {
    pub fn get_ddl_builder(db_type: &DataBaseType) -> Option<Box<dyn DdlBuilder>> {
        match db_type {
            DataBaseType::POSTGRESQL => Some(Box::new(PgDdlClient {})),
            _ => None,
        }
    }
}
