use bmbp_dev_dsl::{BmbpDBSchema, BmbpDBTable};
use crate::ddl::ddl::*;

pub struct MysqlBmbpDbDdlSchemaEngine;

impl BmbpDbDdlSchemaEngine for MysqlBmbpDbDdlSchemaEngine {
    fn create_schema(&self, schema: BmbpDBSchema) -> String {
        let mut sql = String::from("CREATE SCHEMA ");
        sql.push_str(&schema.get_name());
        sql
    }

    fn create_or_replace_schema(&self, schema: BmbpDBSchema) -> String {
        let mut sql = String::from("CREATE OR REPLACE SCHEMA ");
        sql.push_str(&schema.get_name());
        sql
    }

    fn drop_schema(&self, schema: BmbpDBSchema) -> String {
        let mut sql = String::from("DROP SCHEMA ");
        sql.push_str(&schema.get_name());
        sql
    }
}