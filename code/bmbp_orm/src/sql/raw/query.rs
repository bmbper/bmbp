use serde_json::Value;

use bmbp_types::BmbpResp;

use crate::sql::dql::{
    ColumnFieldInner, CstFieldInner, FilterField, FilterType, JoinTable, OrderField, QueryFilter,
    SelectField, Table,
};
use crate::sql::raw::filter::RawFilterBuilder;
use crate::sql::util::{db_alias_escape, db_const_escape, db_escape};
use crate::sql::DynamicSQLParam;
use crate::BmbpQuerySQL;

pub struct RawQueryBuilder<'a> {
    query: &'a BmbpQuerySQL,
    params: &'a DynamicSQLParam,
}

#[allow(dead_code)]
impl<'a> RawQueryBuilder<'a> {
    pub(crate) fn build(&self) -> BmbpResp<(String, Vec<Value>)> {
        let mut raw_sql_parts = vec![];
        let query_select = self.build_query_select(self.get_query().get_select())?;
        raw_sql_parts.push(query_select);
        let query_from = self.build_query_from(self.get_query().get_table())?;
        raw_sql_parts.push(query_from);
        let (query_filter, raw_sql_params) = self.build_filter(self.get_query().get_filter())?;
        if !query_filter.is_empty() {
            raw_sql_parts.push(format!("WHERE {}", query_filter));
        }

        let query_group_by = self.build_query_group_by(self.get_query().get_group())?;
        raw_sql_parts.push(query_group_by);
        let query_order_by = self.build_query_order_by(self.get_query().get_order())?;
        raw_sql_parts.push(query_order_by);
        Ok((raw_sql_parts.join(" "), raw_sql_params))
    }
    #[allow(unused)]
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
    #[allow(unused)]
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
        if raw_order_vec.is_empty() {
            Ok("".to_string())
        } else {
            Ok(format!(" ORDER BY {} ", raw_order_vec.join(",")))
        }
    }

    fn build_filter(&self, filter: Option<&QueryFilter>) -> BmbpResp<(String, Vec<Value>)> {
        if filter.is_none() {
            return Ok(("".to_string(), vec![]));
        }

        let query_filter = filter.unwrap();

        let (raw_filter_field, raw_params) =
            self.build_filter_field(query_filter.get_field_slice())?;

        if raw_filter_field.is_empty() {
            return Ok(("".to_string(), vec![]));
        }

        let filter_type = query_filter.get_filter_type();
        match filter_type {
            FilterType::AND | FilterType::OR => Ok((
                raw_filter_field.join(format!(" {} ", filter_type.to_string()).as_str()),
                raw_params,
            )),
            FilterType::Express(express) => {
                let raw_filter_express =
                    self.build_filter_express(express.clone(), raw_filter_field.as_slice())?;
                Ok((raw_filter_express, raw_params))
            }
        }
    }

    fn build_filter_field(&self, fields: &[FilterField]) -> BmbpResp<(Vec<String>, Vec<Value>)> {
        if fields.is_empty() {
            return Ok((vec![], vec![]));
        }
        let filter_build = RawFilterBuilder::new(fields, self.get_params());
        filter_build.build_filter()?;
        Ok((filter_build.get_raw_fields(), filter_build.get_raw_values()))
    }
    #[allow(unused)]
    fn build_filter_express(&self, express: String, fields: &[String]) -> BmbpResp<String> {
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
        if cst_field.get_field().is_empty() {
            return "".to_string();
        }

        let mut raw_select = "".to_string();

        let select_tag = cst_field.tag();
        if select_tag.is_some() {
            raw_select = select_tag.unwrap().to_string() + " ";
        }

        let select_field = cst_field.get_field();
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
            raw_table = raw_table + "" + "    " + join_sql.as_str();
        }
        Ok(raw_table)
    }

    // TODO 解析JOIN语句
    fn build_query_from_table_join(&self, _join: &JoinTable) -> BmbpResp<String> {
        Ok("".to_string())
    }
}

impl<'a> RawQueryBuilder<'a> {
    pub fn new(query: &'a BmbpQuerySQL, params: &'a DynamicSQLParam) -> RawQueryBuilder<'a> {
        RawQueryBuilder { query, params }
    }

    pub fn get_query(&self) -> &'a BmbpQuerySQL {
        &self.query
    }

    pub fn get_params(&self) -> &'a DynamicSQLParam {
        &self.params
    }
}
