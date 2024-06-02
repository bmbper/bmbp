use crate::{
    RdbcColumnVo, RdbcTableCheckVo, RdbcTableForeignKeyVo, RdbcTableIndexVo, RdbcTablePrimaryKeyVo,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableVo {
    name: String,
    comment: Option<String>,
    charset: Option<String>,
    schema: Option<String>,
    columns: Vec<RdbcColumnVo>,
    primary_key: Option<RdbcTablePrimaryKeyVo>,
    foreign_key: Option<Vec<RdbcTableForeignKeyVo>>,
    check: Option<Vec<RdbcTableCheckVo>>,
    index: Option<Vec<RdbcTableIndexVo>>,
}
impl RdbcTableVo {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn charset(&self) -> &Option<String> {
        &self.charset
    }
    pub fn schema(&self) -> &Option<String> {
        &self.schema
    }
    pub fn columns(&self) -> &Vec<RdbcColumnVo> {
        &self.columns
    }
    pub fn primary_key(&self) -> &Option<RdbcTablePrimaryKeyVo> {
        &self.primary_key
    }
    pub fn foreign_key(&self) -> &Option<Vec<RdbcTableForeignKeyVo>> {
        &self.foreign_key
    }
    pub fn check(&self) -> &Option<Vec<RdbcTableCheckVo>> {
        &self.check
    }
    pub fn index(&self) -> &Option<Vec<RdbcTableIndexVo>> {
        &self.index
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }
    pub fn set_charset(&mut self, charset: Option<String>) {
        self.charset = charset;
    }
    pub fn set_schema(&mut self, schema: Option<String>) {
        self.schema = schema;
    }
    pub fn set_columns(&mut self, columns: Vec<RdbcColumnVo>) {
        self.columns = columns;
    }
    pub fn set_primary_key(&mut self, primary_key: Option<RdbcTablePrimaryKeyVo>) {
        self.primary_key = primary_key;
    }
    pub fn set_foreign_key(&mut self, foreign_key: Option<Vec<RdbcTableForeignKeyVo>>) {
        self.foreign_key = foreign_key;
    }
    pub fn set_check(&mut self, check: Option<Vec<RdbcTableCheckVo>>) {
        self.check = check;
    }
    pub fn set_index(&mut self, index: Option<Vec<RdbcTableIndexVo>>) {
        self.index = index;
    }
}
