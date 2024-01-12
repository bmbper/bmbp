use std::iter::Filter;

pub enum SQLJoinType {
    INNER,
    LEFT,
    RIGHT,
    FULL,
}

pub enum SQLCompareType {}

pub enum SQLValueType {}

/// SQL 语句接口，实现此方法用于生成SQL语句
///
///
pub trait SQL {
    fn to_sql(&self) -> String;
}

pub trait RdbcSQLTable {
    fn get_table_name(&self) -> String;
}

pub trait RdbcSQLColumn {
    fn get_column_name(&self) -> String;
}

impl RdbcSQLTable for &str {
    fn get_table_name(&self) -> String {
        self.to_string()
    }
}

impl RdbcSQLTable for String {
    fn get_table_name(&self) -> String {
        self.to_string()
    }
}


pub struct RdbcTable {
    schema_: Option<String>,
    alias_: Option<String>,
    name_: Box<dyn RdbcSQLTable>,
    join_: Option<SQLJoinType>,
    filter_: Option<QueryFilter>,
}

impl RdbcTable {
    pub fn new(name: &str) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_schema(name: &str, schema: &str) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_alias(name: &str, alias: &str) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_schema_alias(name: &str, schema: &str, alias: &str) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_join(name: &str, join: SQLJoinType) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: None,
        }
    }
    pub fn new_with_schema_join(name: &str, schema: &str, join: SQLJoinType) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: None,
        }
    }
    pub fn new_with_alias_join(name: &str, alias: &str, join: SQLJoinType) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: None,
        }
    }
    pub fn new_with_schema_alias_join(name: &str, schema: &str, alias: &str, join: SQLJoinType) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: None,
        }
    }
    pub fn new_with_schema_filter(name: &str, schema: &str, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: Some(SQLJoinType::INNER),
            filter_: Some(filter),
        }
    }
    pub fn new_with_alias_filter(name: &str, alias: &str, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(SQLJoinType::INNER),
            filter_: Some(filter),
        }
    }
    pub fn new_with_schema_alias_filter(name: &str, schema: &str, alias: &str, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(SQLJoinType::INNER),
            filter_: Some(filter),
        }
    }
    pub fn new_with_join_filter(name: &str, join: SQLJoinType, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: Some(filter),
        }
    }
    pub fn new_with_schema_join_filter(name: &str, schema: &str, join: SQLJoinType, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: Some(filter),
        }
    }
    pub fn new_with_alias_join_filter(name: &str, alias: &str, join: SQLJoinType, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: Some(filter),
        }
    }
    pub fn new_with_schema_alias_join_filter(name: &str, schema: &str, alias: &str, join: SQLJoinType, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: Some(schema.to_string()),
            alias_: Some(alias.to_string()),
            name_: Box::new(name.to_string()),
            join_: Some(join),
            filter_: Some(filter),
        }
    }

    pub fn new_with_query(name: QueryWrapper) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_query_alias(name: QueryWrapper, alias: &str) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: Some(alias.to_string()),
            name_: Box::new(name),
            join_: None,
            filter_: None,
        }
    }
    pub fn new_with_query_join(name: QueryWrapper, join: SQLJoinType) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name),
            join_: Some(join),
            filter_: None,
        }
    }
    pub fn new_with_query_filter(name: QueryWrapper, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name),
            join_: Some(SQLJoinType::INNER),
            filter_: Some(filter),
        }
    }
    pub fn new_with_query_join_filter(name: QueryWrapper, join: SQLJoinType, filter: QueryFilter) -> RdbcTable {
        RdbcTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name),
            join_: Some(join),
            filter_: Some(filter),
        }
    }
}


impl RdbcSQLTable for RdbcTable {
    fn get_table_name(&self) -> String {
        "".to_string()
    }
}


pub struct RdbcColumn {
    schema_: Option<String>,
    table_: Option<String>,
    alias_: Option<String>,
    name: Box<dyn RdbcSQLColumn>,
}

pub struct RdbcCompareColumn {
    column: RdbcColumn,
    compare_: SQLCompareType,
    value_: SQLValueType,
}


pub enum SQLWrapper {
    Insert(InsertWrapper),
    Query(QueryWrapper),
    UPDATE(UpdateWrapper),
    DELETE(DeleteWrapper),
}

pub struct InsertWrapper {}

pub struct UpdateWrapper {}

pub struct QueryWrapper {}

impl RdbcSQLTable for QueryWrapper {
    fn get_table_name(&self) -> String {
        "".to_string()
    }
}

pub struct DeleteWrapper {}


pub struct QueryFilter {}


#[cfg(test)]
pub mod tests {
    use crate::wrapper::{RdbcSQLTable, RdbcTable};

    #[test]
    fn test_table_model() {
        let table = RdbcTable::new("a");
        assert_eq!("a", table.get_table_name())
    }
}