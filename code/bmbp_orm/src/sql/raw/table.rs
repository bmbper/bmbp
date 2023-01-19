use bmbp_types::{BmbpError, BmbpResp};

use crate::sql::dql::Table;

#[allow(dead_code)]
pub enum TableBuilderType {
    QUERY,
    INSERT,
    UPDATE,
    DELETE,
}
pub struct TableBuilder<'a> {
    tables: &'a [Table],
    typ: TableBuilderType,
}
#[allow(dead_code)]
impl<'a> TableBuilder<'a> {
    pub fn query(tables: &'a [Table]) -> Self {
        TableBuilder {
            tables,
            typ: TableBuilderType::QUERY,
        }
    }
    pub fn update(tables: &'a [Table]) -> Self {
        TableBuilder {
            tables,
            typ: TableBuilderType::UPDATE,
        }
    }
    pub fn insert(tables: &'a [Table]) -> Self {
        TableBuilder {
            tables,
            typ: TableBuilderType::INSERT,
        }
    }
    pub fn delete(tables: &'a [Table]) -> Self {
        TableBuilder {
            tables,
            typ: TableBuilderType::DELETE,
        }
    }
    pub fn get_type(&self) -> &TableBuilderType {
        &self.typ
    }
    pub fn get_table(&self) -> &[Table] {
        self.tables
    }
}

#[allow(dead_code)]
impl<'a> TableBuilder<'a> {
    pub fn build(&self) -> BmbpResp<String> {
        match self.get_type() {
            TableBuilderType::QUERY => self.build_dql(),
            TableBuilderType::INSERT | TableBuilderType::UPDATE | TableBuilderType::DELETE => {
                self.build_dml()
            }
        }
    }

    fn build_dql(&self) -> BmbpResp<String> {
        self.valid()?;
        Ok("".to_string())
    }

    fn build_dml(&self) -> BmbpResp<String> {
        self.valid()?;
        self.build_dml_table_slice()
    }
    fn valid(&self) -> BmbpResp<()> {
        let table_slice = self.get_table();
        if table_slice.is_empty() {
            return Err(BmbpError::dyn_sql("表结构不能为空".to_string()));
        }
        Ok(())
    }
    fn build_dml_table_slice(&self) -> BmbpResp<String> {
        let mut raw_table_vec = vec![];
        for table in self.get_table() {
            let raw_table = self.build_dml_table(table)?;
            raw_table_vec.push(raw_table);
        }
        Ok(raw_table_vec.join(","))
    }
    fn build_dml_table(&self, table: &Table) -> BmbpResp<String> {
        let mut raw_table = table.table_name().clone();
        if !table.table_alias().is_empty() {
            raw_table = raw_table + " AS " + table.table_alias();
        }
        if raw_table.is_empty() {
            return Err(BmbpError::dyn_sql("表名称不能为空".to_string()));
        }
        Ok(raw_table)
    }
}
