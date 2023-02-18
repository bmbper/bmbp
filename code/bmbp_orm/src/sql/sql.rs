use bmbp_types::{BmbpError, BmbpResp};

use super::{
    ddl::DdlSQL,
    dml::{DeleteSQL, InsertSQL, UpdateSQL},
    dql::QuerySQL,
};
#[derive(Clone, Debug)]
pub enum DynamicSQL {
    Query(QuerySQL),
    Insert(InsertSQL),
    Update(UpdateSQL),
    Delete(DeleteSQL),
    DDL(DdlSQL),
}

impl DynamicSQL {
    pub fn new() -> DynamicSQL {
        DynamicSQL::Query(QuerySQL::new())
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

impl DynamicSQL {
    pub fn as_query_mut(&mut self) -> BmbpResp<&mut QuerySQL> {
        match self {
            DynamicSQL::Query(query) => Ok(query),
            _ => Err(BmbpError::orm("不是查询类型SQL".to_string())),
        }
    }

    pub fn as_insert_mut(&mut self) -> BmbpResp<&mut InsertSQL> {
        match self {
            DynamicSQL::Insert(insert) => Ok(insert),
            _ => Err(BmbpError::orm("不是新增类型SQL".to_string())),
        }
    }

    pub fn as_update_mut(&mut self) -> BmbpResp<&mut UpdateSQL> {
        match self {
            DynamicSQL::Update(update) => Ok(update),
            _ => Err(BmbpError::orm("不是更新类型SQL".to_string())),
        }
    }

    pub fn as_delete_mut(&mut self) -> BmbpResp<&mut DeleteSQL> {
        match self {
            DynamicSQL::Delete(delete) => Ok(delete),
            _ => Err(BmbpError::orm("不是删除类型SQL".to_string())),
        }
    }

    pub fn is_query(&self) -> bool {
        match self {
            DynamicSQL::Query(_) => true,
            _ => false,
        }
    }

    pub fn is_insert(&self) -> bool {
        match self {
            DynamicSQL::Insert(_) => true,
            _ => false,
        }
    }

    pub fn is_update(&self) -> bool {
        match self {
            DynamicSQL::Update(_) => true,
            _ => false,
        }
    }

    pub fn is_delete(&self) -> bool {
        match self {
            DynamicSQL::Delete(_) => true,
            _ => false,
        }
    }

    pub fn is_ddl(&self) -> bool {
        match self {
            DynamicSQL::DDL(_) => true,
            _ => false,
        }
    }

    pub fn to_orm_sql(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::DynamicSQL;

    #[test]
    fn test_sql() {
        let mut query = DynamicSQL::query();
    }
}
