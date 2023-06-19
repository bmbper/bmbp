use serde_json::Value;

use crate::sql::raw::{
    RawDDLBuilder, RawDeleteBuilder, RawInsertBuilder, RawQueryBuilder, RawUpdateBuilder,
};
use crate::sql::DdlSQL;
use crate::{BmbpDeleteSQL, BmbpInsertSQL, BmbpQuerySQL, BmbpUpdateSQL};
use bmbp_app_common::BmbpResp;

use super::{param::DynamicSQLParam, sql::BmbpDynamicSQL};

#[derive(Debug)]
pub struct BmbpOrmSQL {
    dynamic_sql: BmbpDynamicSQL,
    dynamic_param: DynamicSQLParam,
    raw_sql: String,
    raw_sql_params: Vec<Value>,
}

impl BmbpOrmSQL {
    pub fn query() -> Self {
        let dynamic_sql = BmbpDynamicSQL::Query(BmbpDynamicSQL::query());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = BmbpOrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn update() -> Self {
        let dynamic_sql = BmbpDynamicSQL::Update(BmbpDynamicSQL::update());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = BmbpOrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn insert() -> Self {
        let dynamic_sql = BmbpDynamicSQL::Insert(BmbpDynamicSQL::insert());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = BmbpOrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
    pub fn delete() -> Self {
        let dynamic_sql = BmbpDynamicSQL::Delete(BmbpDynamicSQL::delete());
        let dynamic_params = DynamicSQLParam::new();
        let orm_sql = BmbpOrmSQL {
            dynamic_sql,
            dynamic_param: dynamic_params,
            raw_sql_params: vec![],
            raw_sql: "".to_string(),
        };
        orm_sql
    }
}

impl BmbpOrmSQL {
    pub fn set_dynamic_sql(&mut self, sql: BmbpDynamicSQL) -> &mut Self {
        self.dynamic_sql = sql;
        self
    }
    pub fn get_dynamic_sql(&self) -> &BmbpDynamicSQL {
        &self.dynamic_sql
    }
    pub fn get_mut_dynamic_sql(&mut self) -> &mut BmbpDynamicSQL {
        &mut self.dynamic_sql
    }
    pub fn set_dynamic_params(&mut self, param: DynamicSQLParam) -> &mut Self {
        self.dynamic_param = param;
        self
    }
    pub fn get_dynamic_params(&self) -> &DynamicSQLParam {
        &self.dynamic_param
    }
    pub fn get_mut_dynamic_params(&mut self) -> &mut DynamicSQLParam {
        &mut self.dynamic_param
    }
    pub fn set_raw_sql(&mut self, raw_sql: String) -> &mut Self {
        self.raw_sql = raw_sql;
        self
    }
    pub fn set_raw_sql_params(&mut self, raw_sql_params: Vec<Value>) -> &mut Self {
        self.raw_sql_params = raw_sql_params;
        self
    }
    pub fn set_raw(&mut self, raw_sql: String, raw_sql_params: Vec<Value>) -> &mut Self {
        self.raw_sql = raw_sql;
        self.raw_sql_params = raw_sql_params;
        self
    }

    pub fn as_query_mut(&mut self) -> BmbpResp<&mut BmbpQuerySQL> {
        self.dynamic_sql.as_query_mut()
    }
    pub fn as_insert_mut(&mut self) -> BmbpResp<&mut BmbpInsertSQL> {
        self.dynamic_sql.as_insert_mut()
    }
    pub fn as_update_mut(&mut self) -> BmbpResp<&mut BmbpUpdateSQL> {
        self.dynamic_sql.as_update_mut()
    }
    pub fn as_delete_mut(&mut self) -> BmbpResp<&mut BmbpDeleteSQL> {
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

impl BmbpOrmSQL {
    pub fn get_raw_orm(&mut self) -> BmbpResp<(String, &[Value])> {
        self.build_raw_orm()?;
        Ok((self.raw_sql.clone(), self.raw_sql_params.as_slice()))
    }
}

impl BmbpOrmSQL {
    fn build_raw_orm(&mut self) -> BmbpResp<()> {
        let (raw_sql, raw_params) = match self.get_dynamic_sql() {
            BmbpDynamicSQL::Query(query) => self.build_query(query)?,
            BmbpDynamicSQL::Insert(insert) => self.build_insert(insert)?,
            BmbpDynamicSQL::Update(update) => self.build_update(update)?,
            BmbpDynamicSQL::Delete(delete) => self.build_delete(delete)?,
            BmbpDynamicSQL::DDL(ddl) => self.build_ddl(ddl)?,
        };
        self.set_raw(raw_sql, raw_params);
        Ok(())
    }

    fn build_query(&self, query: &BmbpQuerySQL) -> BmbpResp<(String, Vec<Value>)> {
        RawQueryBuilder::new(query, self.get_dynamic_params()).build()
    }

    fn build_insert(&self, insert: &BmbpInsertSQL) -> BmbpResp<(String, Vec<Value>)> {
        RawInsertBuilder::new(insert, self.get_dynamic_params()).build()
    }

    fn build_update(&self, update: &BmbpUpdateSQL) -> BmbpResp<(String, Vec<Value>)> {
        RawUpdateBuilder::new(update, self.get_dynamic_params()).build()
    }

    fn build_delete(&self, delete: &BmbpDeleteSQL) -> BmbpResp<(String, Vec<Value>)> {
        RawDeleteBuilder::new(delete, self.get_dynamic_params()).build()
    }

    fn build_ddl(&self, ddl: &DdlSQL) -> BmbpResp<(String, Vec<Value>)> {
        RawDDLBuilder::new(ddl, self.get_dynamic_params()).build()
    }
}
