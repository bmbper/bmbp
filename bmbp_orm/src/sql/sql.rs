use bmbp_types::{BmbpError, BmbpResp};

use super::{
    ddl::DdlSQL,
    dml::{DeleteSQL, InsertSQL, UpdateSQL},
    dql::QuerySQL,
};

pub enum SQL {
    Query(QuerySQL),
    Insert(InsertSQL),
    Update(UpdateSQL),
    Delete(DeleteSQL),
    DDL(DdlSQL),
}

impl SQL {
    pub fn new() -> SQL {
        SQL::Query(QuerySQL::new())
    }

    pub fn query() -> QuerySQL {
        QuerySQL::new()
    }

    pub fn insert() -> InsertSQL {
        InsertSQL::new()
    }

    pub fn update() -> UpdateSQL {
        UpdateSQL::new()
    }

    pub fn delete() -> DeleteSQL {
        DeleteSQL::new()
    }
}

impl SQL {
    pub fn as_query_mut(&mut self) -> BmbpResp<&mut QuerySQL> {
        match self {
            SQL::Query(query) => Ok(query),
            _ => Err(BmbpError::orm("不是查询类型SQL".to_string())),
        }
    }

    pub fn to_orm_sql(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SQL;

    #[test]
    fn test_sql() {
        let mut query = SQL::query();
    }
}
