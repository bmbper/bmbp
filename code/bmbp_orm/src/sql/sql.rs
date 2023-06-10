use bmbp_types::{BmbpError, BmbpResp};

use super::{
    ddl::DdlSQL,
    dml::{BmbpDeleteSQL, BmbpInsertSQL, BmbpUpdateSQL},
    dql::BmbpQuerySQL,
};
#[derive(Clone, Debug)]
pub enum BmbpDynamicSQL {
    Query(BmbpQuerySQL),
    Insert(BmbpInsertSQL),
    Update(BmbpUpdateSQL),
    Delete(BmbpDeleteSQL),
    DDL(DdlSQL),
}

impl BmbpDynamicSQL {
    pub fn new() -> BmbpDynamicSQL {
        BmbpDynamicSQL::Query(BmbpQuerySQL::new())
    }
    pub fn query() -> BmbpQuerySQL {
        BmbpQuerySQL::new()
    }
    pub fn insert() -> BmbpInsertSQL {
        BmbpInsertSQL::new()
    }
    pub fn update() -> BmbpUpdateSQL {
        BmbpUpdateSQL::new()
    }
    pub fn delete() -> BmbpDeleteSQL {
        BmbpDeleteSQL::new()
    }
}

impl BmbpDynamicSQL {
    pub fn as_query_mut(&mut self) -> BmbpResp<&mut BmbpQuerySQL> {
        match self {
            BmbpDynamicSQL::Query(query) => Ok(query),
            _ => Err(BmbpError::orm("不是查询类型SQL".to_string())),
        }
    }

    pub fn as_insert_mut(&mut self) -> BmbpResp<&mut BmbpInsertSQL> {
        match self {
            BmbpDynamicSQL::Insert(insert) => Ok(insert),
            _ => Err(BmbpError::orm("不是新增类型SQL".to_string())),
        }
    }

    pub fn as_update_mut(&mut self) -> BmbpResp<&mut BmbpUpdateSQL> {
        match self {
            BmbpDynamicSQL::Update(update) => Ok(update),
            _ => Err(BmbpError::orm("不是更新类型SQL".to_string())),
        }
    }

    pub fn as_delete_mut(&mut self) -> BmbpResp<&mut BmbpDeleteSQL> {
        match self {
            BmbpDynamicSQL::Delete(delete) => Ok(delete),
            _ => Err(BmbpError::orm("不是删除类型SQL".to_string())),
        }
    }

    pub fn is_query(&self) -> bool {
        match self {
            BmbpDynamicSQL::Query(_) => true,
            _ => false,
        }
    }

    pub fn is_insert(&self) -> bool {
        match self {
            BmbpDynamicSQL::Insert(_) => true,
            _ => false,
        }
    }

    pub fn is_update(&self) -> bool {
        match self {
            BmbpDynamicSQL::Update(_) => true,
            _ => false,
        }
    }

    pub fn is_delete(&self) -> bool {
        match self {
            BmbpDynamicSQL::Delete(_) => true,
            _ => false,
        }
    }

    pub fn is_ddl(&self) -> bool {
        match self {
            BmbpDynamicSQL::DDL(_) => true,
            _ => false,
        }
    }

    pub fn to_orm_sql(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::BmbpDynamicSQL;

    #[test]
    fn test_sql() {
        _ = BmbpDynamicSQL::query();
    }
}
