use std::collections::HashMap;
use std::rc::Weak;

use serde_json::Value;

use bmbp_types::{BmbpError, BmbpResp};

use crate::{DeleteSQL, InsertSQL, QuerySQL, UpdateSQL};

use super::{param::DynamicSQLParam, parse::parse_orm_sql, sql::DynamicSQL};

pub struct OrmSQL {
    dynamic_sql: DynamicSQL,
    dynamic_param: DynamicSQLParam,
    raw_sql: String,
    raw_sql_params: Vec<Value>,
}

impl OrmSQL {
    pub fn query() -> Self {
        let dynamic_sql = DynamicSQL::Query(DynamicSQL::query());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = OrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn update() -> Self {
        let dynamic_sql = DynamicSQL::Update(DynamicSQL::update());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = OrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn insert() -> Self {
        let dynamic_sql = DynamicSQL::Insert(DynamicSQL::insert());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = OrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn delete() -> Self {
        let dynamic_sql = DynamicSQL::Delete(DynamicSQL::delete());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = OrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
}

impl OrmSQL {
    pub fn set_dynamic_sql(&mut self, sql: DynamicSQL) -> &mut Self {
        self.dynamic_sql = sql;
        self
    }
    pub fn get_dynamic_sql(&self) -> &DynamicSQL {
        &self.dynamic_sql
    }
    pub fn set_dynamic_params(&mut self, param: DynamicSQLParam) -> &mut Self {
        self.dynamic_param = param;
        self
    }
    pub fn get_dynamic_params(&self) -> &DynamicSQLParam {
        &self.dynamic_param
    }
    pub fn set_raw_sql(&mut self, raw_sql: String) -> &mut Self {
        self.raw_sql = raw_sql;
        self
    }
    pub fn set_raw_sql_params(&mut self, raw_sql_params: Vec<Value>) -> &mut Self {
        self.raw_sql_params = raw_sql_params;
        self
    }
    pub fn as_query_mut(&mut self) -> BmbpResp<&mut QuerySQL> {
        self.dynamic_sql.as_query_mut()
    }
    pub fn as_insert_mut(&mut self) -> BmbpResp<&mut InsertSQL> {
        self.dynamic_sql.as_insert_mut()
    }
    pub fn as_update_mut(&mut self) -> BmbpResp<&mut UpdateSQL> {
        self.dynamic_sql.as_update_mut()
    }
    pub fn as_delete_mut(&mut self) -> BmbpResp<&mut DeleteSQL> {
        self.dynamic_sql.as_delete_mut()
    }
    pub fn is_query(&self) -> bool {
        self.dynamic_sql.is_query()
    }
    pub fn is_insert(&self) -> bool {
        self.dynamic_sql.is_insert()
    }
    pub fn is_update(&self) -> bool {
        self.dynamic_sql.is_update()
    }
    pub fn is_delete(&self) -> bool {
        self.dynamic_sql.is_delete()
    }
}

impl OrmSQL {
    pub fn to_orm_sql(&mut self) -> BmbpResp<(String, &[Value])> {
        parse_orm_sql(self)?;
        Ok((self.raw_sql.clone(), self.raw_sql_params.as_slice()))
    }
}
