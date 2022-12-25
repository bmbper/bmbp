use serde_json::Value;

use bmbp_types::{BmbpError, BmbpResp};

use crate::QuerySQL;

use super::{param::SQLParam, parse::parse_orm_sql, sql::SQL};

pub struct OrmSQL {
    orm_sql: Option<SQL>,
    orm_param: Option<SQLParam>,
    sql_param: Vec<Value>,
    sql: String,
}

impl OrmSQL {
    pub fn new() -> Self {
        OrmSQL {
            orm_sql: None,
            orm_param: None,
            sql_param: vec![],
            sql: "".to_string(),
        }
    }

    pub fn orm_params(&self) -> Option<&SQLParam> {
        self.orm_param.as_ref()
    }

    pub fn orm_sql(&self) -> Option<&SQL> {
        self.orm_sql.as_ref()
    }
    pub fn orm_sql_mut(&mut self) -> Option<&mut SQL> {
        self.orm_sql.as_mut()
    }
}

impl OrmSQL {
    pub fn query(&mut self) -> BmbpResp<&mut QuerySQL> {
        if self.orm_sql.is_none() {
            self.orm_sql = Some(SQL::Query(QuerySQL::new()));
        }
        self.orm_sql.as_mut().unwrap().as_query_mut()
    }
}

impl OrmSQL {
    pub fn set_orm_sql(&mut self, sql: SQL) -> &mut Self {
        self.orm_sql = Some(sql);
        self
    }
    pub fn set_orm_params(&mut self, param: SQLParam) -> &mut Self {
        self.orm_param = Some(param);
        self
    }
    pub fn set_sql(&mut self, sql: String) -> &mut Self {
        self.sql = sql;
        self
    }
    pub fn set_params(&mut self, param: Vec<Value>) -> &mut Self {
        self.sql_param = param;
        self
    }
    pub fn to_orm_sql(&mut self) -> BmbpResp<(String, &[Value])> {
        if self.orm_sql.is_none() {
            return Err(BmbpError::orm("请配置SQL".to_string()));
        }
        parse_orm_sql(self)?;
        Ok((self.sql.clone(), self.sql_param.as_slice()))
    }
}
