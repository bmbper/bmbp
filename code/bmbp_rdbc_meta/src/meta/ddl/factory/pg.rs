use crate::meta::ddl::factory::DdlBuilder;
use crate::{BmbpRdbcColumn, BmbpRdbcTable};
pub struct PgDdlClient {}

impl PgDdlClient {
    pub(crate) fn build_create_table(&self, table: &BmbpRdbcTable) -> String {
        if let Some(schema) = table.schema() {
            format!("{}.{}", schema, table.name())
        } else {
            format!("{}", table.name())
        }
    }
    pub(crate) fn build_create_columns(&self, table: &BmbpRdbcTable) -> Vec<String> {
        let mut column_sql = vec![];
        let columns = table.columns();
        for item in columns {
            let column_name = self.build_create_column(item);
            column_sql.push(column_name);
        }
        column_sql
    }
    pub(crate) fn build_create_column(&self, column: &BmbpRdbcColumn) -> String {
        "".to_string()
    }
    pub(crate) fn build_create_primary_key(&self, table: &BmbpRdbcTable) -> Option<String> {
        None
    }
    pub(crate) fn build_create_table_index(&self, table: &BmbpRdbcTable) -> Vec<String> {
        vec![]
    }
    pub(crate) fn build_create_table_comment(&self, table: &BmbpRdbcTable) -> Vec<String> {
        vec![]
    }
    pub(crate) fn build_create_table_foreign_key(&self, table: &BmbpRdbcTable) -> Vec<String> {
        vec![]
    }
}

impl DdlBuilder for PgDdlClient {
    fn build_create_table_sql(&self, table: &BmbpRdbcTable) -> Result<String, String> {
        let table_name = self.build_create_table(table);
        let columns = self.build_create_columns(table);
        let primary_key = self.build_create_primary_key(table);
        let table_index = self.build_create_table_index(table);
        let table_comment = self.build_create_table_comment(table);
        let table_foreign_key = self.build_create_table_foreign_key(table);
        let mut sql = vec![];
        let create_table_sql = format!("CREATE TABLE {}({}) ", table_name, columns.join(","));
        sql.push(create_table_sql);
        if let Some(primary) = primary_key {
            sql.push(primary);
        }
        sql.extend_from_slice(table_index.as_slice());
        sql.extend_from_slice(table_comment.as_slice());
        sql.extend_from_slice(table_foreign_key.as_slice());
        Ok(sql.join(";"))
    }
}
