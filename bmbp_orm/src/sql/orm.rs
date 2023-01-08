use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::rc::Weak;

use serde_json::Value;

use bmbp_types::{BmbpError, BmbpResp};

use crate::sql::dml::DmlField;
use crate::sql::dql::{
    ColumnFieldInner, ComplexFilterInner, CstFieldInner, FilterField, FilterType, JoinTable,
    OrderField, QueryFilter, SelectField, SimpleFilterInner, Table,
};
use crate::sql::orm_filter::FilterBuilder;
use crate::sql::util::{db_alias_escape, db_const_escape, db_escape};
use crate::sql::DdlSQL;
use crate::{DeleteSQL, InsertSQL, QuerySQL, UpdateSQL};

use super::{param::DynamicSQLParam, sql::DynamicSQL};

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
    pub fn get_mut_dynamic_sql(&mut self) -> &mut DynamicSQL {
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
    pub fn to_raw_sql_params(&mut self) -> BmbpResp<(String, &[Value])> {
        self.build_raw_sql_params()?;
        Ok((self.raw_sql.clone(), self.raw_sql_params.as_slice()))
    }
}

impl OrmSQL {
    fn build_raw_sql_params(&mut self) -> BmbpResp<()> {
        let (raw_sql, raw_params) = match self.get_dynamic_sql() {
            DynamicSQL::Query(query) => self.build_query(query)?,
            DynamicSQL::Insert(insert) => self.build_insert(insert)?,
            DynamicSQL::Update(update) => self.build_update(update)?,
            DynamicSQL::Delete(delete) => self.build_delete(delete)?,
            DynamicSQL::DDL(ddl) => self.build_ddl(ddl)?,
        };
        self.set_raw(raw_sql, raw_params);
        Ok(())
    }

    fn build_query(&self, query: &QuerySQL) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_sql_parts = vec![];

        let query_select = self.build_query_select(query.get_select())?;
        raw_sql_parts.push(query_select);

        let query_from = self.build_query_from(query.get_table())?;
        raw_sql_parts.push(query_from);

        let (query_filter, raw_sql_params) = self.build_filter(query.get_filter())?;
        raw_sql_parts.push(query_filter);

        let query_group_by = self.build_query_group_by(query.get_group())?;
        raw_sql_parts.push(query_group_by);

        let query_order_by = self.build_query_order_by(query.get_order())?;
        raw_sql_parts.push(query_order_by);

        Ok((raw_sql_parts.join(" \n"), raw_sql_params))
    }

    fn build_insert(&self, query: &InsertSQL) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }

    fn build_update(&self, update: &UpdateSQL) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }

    fn build_delete(&self, delete: &DeleteSQL) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }

    fn build_ddl(&self, ddl: &DdlSQL) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }

    fn build_query_select(&self, fields: &[SelectField]) -> BmbpResp<String> {
        let mut select_vec = vec![];
        for field in fields {
            match field {
                SelectField::COLUMN(column) => {
                    let field = self.build_query_select_column_field(column);
                    if !field.is_empty() {
                        select_vec.push(field);
                    }
                }
                SelectField::CST(cst) => {
                    let field = self.build_query_select_cst_field(cst);
                    if !field.is_empty() {
                        select_vec.push(field);
                    }
                }
                SelectField::FUNC(func) => {}
                SelectField::EXPRESS(express) => {}
                SelectField::SQL(sql) => {}
            }
        }

        if select_vec.is_empty() {
            Ok("".to_string())
        } else {
            Ok("SELECT ".to_string() + select_vec.join(",").as_str())
        }
    }

    fn build_query_from(&self, tables: &[Table]) -> BmbpResp<String> {
        let mut raw_table_vec = vec![];
        for table in tables {
            let raw_table = self.build_query_from_table(table)?;
            if !raw_table.is_empty() {
                raw_table_vec.push(raw_table);
            }
        }
        if raw_table_vec.is_empty() {
            Ok("".to_string())
        } else {
            Ok(" FROM ".to_string() + raw_table_vec.join(",").as_str())
        }
    }

    fn build_query_group_by(&self, fields: &[SelectField]) -> BmbpResp<String> {
        Ok("".to_string())
    }

    fn build_query_order_by(&self, fields: &[OrderField]) -> BmbpResp<String> {
        let mut raw_order_vec = vec![];
        for order_field in fields {
            raw_order_vec.push(format!(
                "{} {}",
                order_field.get_field(),
                order_field.get_order_type().to_string()
            ));
        }
        Ok(raw_order_vec.join(","))
    }

    fn build_filter(&self, filter: Option<&QueryFilter>) -> BmbpResp<(String, Vec<Value>)> {
        if filter.is_none() {
            return Ok(("".to_string(), vec![]));
        }

        let query_filter = filter.unwrap();
        match query_filter {
            QueryFilter::Complex(complex) => self.build_complex_filter(complex),
            QueryFilter::Simple(simple) => self.build_simple_filter(simple),
        }
    }

    fn build_simple_filter(&self, simple: &SimpleFilterInner) -> BmbpResp<(String, Vec<Value>)> {
        let (raw_filter_field, raw_params) =
            self.build_simple_filter_field(simple.get_field_slice())?;
        if raw_filter_field.is_empty() {
            return Ok(("".to_string(), vec![]));
        }

        let filter_type = simple.get_filter_type();
        match filter_type {
            FilterType::AND | FilterType::OR => Ok((
                raw_filter_field.join(filter_type.to_string().as_str()),
                raw_params,
            )),
            FilterType::Express => {
                let express = simple.get_raw_express();
                let raw_filter_express =
                    self.build_simple_filter_express(express, raw_filter_field.as_slice())?;
                Ok((raw_filter_express, raw_params))
            }
        }
    }

    fn build_simple_filter_field(
        &self,
        fields: &[FilterField],
    ) -> BmbpResp<(Vec<String>, Vec<Value>)> {
        if fields.is_empty() {
            return Ok((vec![], vec![]));
        }
        let filter_build = FilterBuilder::new(fields, self.get_dynamic_params());
        filter_build.build_filter()
    }

    fn build_simple_filter_express(&self, express: String, fields: &[String]) -> BmbpResp<String> {
        Ok("".to_string())
    }

    fn build_complex_filter(&self, complex: &ComplexFilterInner) -> BmbpResp<(String, Vec<Value>)> {
        Ok(("".to_string(), vec![]))
    }

    fn build_insert_field(&self, fields: &[DmlField]) -> BmbpResp<String> {
        Ok("".to_string())
    }

    fn build_update_field(&self, fields: &[DmlField]) -> BmbpResp<String> {
        Ok("".to_string())
    }
    fn build_query_select_column_field(&self, field_inner: &ColumnFieldInner) -> String {
        if field_inner.field().is_empty() {
            return "".to_string();
        }

        let mut raw_select = "".to_string();

        let select_tag = field_inner.tag();
        if select_tag.is_some() {
            raw_select = select_tag.unwrap().to_string() + " ";
        }

        let table_alias = field_inner.table_alias();
        if !table_alias.is_empty() {
            raw_select = raw_select + table_alias + ".";
        }

        let select_field = field_inner.field();
        raw_select = raw_select + db_escape(select_field).as_str();

        let select_alias = field_inner.alias();
        if !select_alias.is_empty() {
            raw_select = raw_select + " AS " + db_alias_escape(select_alias).as_str();
        }

        raw_select
    }

    fn build_query_select_cst_field(&self, cst_field: &CstFieldInner) -> String {
        if cst_field.get_filed().is_empty() {
            return "".to_string();
        }

        let mut raw_select = "".to_string();

        let select_tag = cst_field.tag();
        if select_tag.is_some() {
            raw_select = select_tag.unwrap().to_string() + " ";
        }

        let select_field = cst_field.get_filed();
        raw_select = raw_select + db_const_escape(select_field).as_str();

        let select_alias = cst_field.get_alias();
        if !select_alias.is_empty() {
            raw_select = raw_select + " AS " + db_alias_escape(select_alias).as_str();
        }

        raw_select
    }

    fn build_query_from_table(&self, table: &Table) -> BmbpResp<String> {
        let mut raw_table = table.table_name().clone();

        if !table.table_alias().is_empty() {
            raw_table = raw_table + " AS " + table.table_alias();
        }

        if table.join().is_some() {
            let join_sql = self.build_query_from_table_join(table.join().unwrap())?;
            raw_table = raw_table + "\n" + "    " + join_sql.as_str();
        }
        Ok(raw_table)
    }

    // TODO 解析JOIN语句
    fn build_query_from_table_join(&self, p0: &JoinTable) -> BmbpResp<String> {
        Ok("".to_string())
    }
}
