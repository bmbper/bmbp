use crate::{BmbpRdbcSQLJoinType, RdbcSQLFunc, RdbcSQLTable, RdbcSQLValue, RdbcValue};

/// SQL 语句接口，实现此方法用于生成SQL语句
pub enum SQLCompareType {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}


pub struct RdbcColumn {
    schema_: Option<String>,
    table_: Option<String>,
    alias_: Option<String>,
}

pub struct RdbcCompareColumn {
    column: RdbcColumn,
    compare_: SQLCompareType,
    value_: RdbcValue,
}

pub enum RdbcFunc {}

pub enum SQLWrapper {
    Insert(InsertWrapper),
    Query(QueryWrapper),
    UPDATE(UpdateWrapper),
    DELETE(DeleteWrapper),
}

pub struct InsertWrapper {}

pub struct UpdateWrapper {}

pub struct QueryWrapper {}

impl QueryWrapper {
    pub fn new() -> Self {
        QueryWrapper {}
    }

    pub fn select(&mut self, field: &str) -> &mut Self {
        self
    }
    pub fn select_with_table(&mut self, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn select_with_schema_table(&mut self, schema: &str, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn select_with_alias(&mut self, field: &str) -> &mut Self {
        self
    }
    pub fn select_with_table_alias(&mut self, table: &str, field: &str, alias: &str) -> &mut Self {
        self
    }
    pub fn select_with_schema_table_alias(&mut self, schema: &str, table: &str, field: &str, alias: &str) -> &mut Self {
        self
    }

    pub fn select_with_query_alias(&mut self, query: QueryWrapper, alias: &str) -> &mut Self {
        self
    }

    pub fn select_with_sql_value(&mut self, value: Box<dyn RdbcSQLValue>) -> &mut Self {
        self
    }
    pub fn select_with_sql_value_alias(&mut self, value: Box<dyn RdbcSQLValue>, alias: &str) -> &mut Self {
        self
    }
    pub fn select_with_value(&mut self, value: RdbcValue) -> &mut Self {
        self
    }
    pub fn select_with_value_alias(&mut self, value: RdbcValue) -> &mut Self {
        self
    }
    pub fn select_with_column(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    pub fn select_with_sql_func(&mut self, func: Box<dyn RdbcSQLFunc>) {}
    pub fn select_with_sql_func_alias(&mut self, func: Box<dyn RdbcSQLFunc>, alias: &str) -> &mut Self {
        self
    }
    pub fn select_with_func(&mut self, func: RdbcFunc) {}
    pub fn select_with_func_alias(&mut self, func: RdbcFunc, alias: &str) -> &mut Self {
        self
    }
    pub fn table(&mut self, table: &str) -> &mut Self {
        self
    }
    pub fn table_with_schema(&mut self, table: &str, alias: &str) -> &mut Self {
        self
    }
    pub fn table_with_alise(&mut self, table: &str, alias: &str) -> &mut Self {
        self
    }
    pub fn table_with_query(&mut self, table: &str, query: QueryWrapper) -> &mut Self {
        self
    }
    pub fn table_with_query_alias(&mut self, table: &str, alias: &str, query: QueryWrapper) -> &mut Self {
        self
    }
    pub fn join_table(&mut self, table: &str, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }
    pub fn join_table_with_alias(&mut self, table: &str, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }
    pub fn join_table_with_schema(&mut self, table: &str, schema: &str, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }
    pub fn join_table_with_schema_alias(&mut self, table: &str, schema: &str, alias: &str, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }

    pub fn join_table_with_query(&mut self, table: QueryWrapper, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }
    pub fn join_table_alias_with_query(&mut self, table: QueryWrapper, alias: &str, join: BmbpRdbcSQLJoinType) -> &mut Self {
        self
    }

    pub fn filter(&mut self, filter: QueryFilter) -> &mut Self {
        self
    }

    pub fn order_asc(&mut self, field: &str) -> &mut Self {
        self
    }
    pub fn order_asc_with_table(&mut self, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn order_asc_with_schema_table(&mut self, schema: &str, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn order_desc(&mut self, field: &str) -> &mut Self {
        self
    }
    pub fn order_desc_with_table(&mut self, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn order_desc_with_schema_table(&mut self, schema: &str, table: &str, field: &str) -> &mut Self {
        self
    }
    pub fn group_by(&mut self, field: &str) -> &mut Self { self }
    pub fn group_by_with_table(&mut self, table: &str, field: &str) -> &mut Self { self }
    pub fn group_by_with_schema_table(&mut self, schema: &str, table: &str, field: &str) -> &mut Self { self }
    pub fn having(&mut self, field: RdbcCompareColumn) -> &mut Self {
        self
    }
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self
    }
    pub fn limit_offset(&mut self, limit: i32) -> &mut Self {
        self
    }
}

impl RdbcSQLTable for QueryWrapper {
    fn to_table(&self) -> String {
        "".to_string()
    }
}

pub struct DeleteWrapper {}

pub struct QueryFilter {}


#[cfg(test)]
pub mod tests {
    use crate::dql::query::QueryWrapper;

    #[test]
    fn test_table_model() {
        let mut query = QueryWrapper::new();
    }
}