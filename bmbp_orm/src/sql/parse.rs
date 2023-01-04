use serde_json::Value;

use bmbp_types::{BmbpError, BmbpResp};

use crate::{DeleteSQL, DynamicSQL, InsertSQL, OrmSQL, QuerySQL, UpdateSQL};

use super::{
    dql::{
        ColumnFieldInner, CstFieldInner, JoinTable, OrderField, QueryFilter, SelectField, Table,
    },
    DdlSQL, DynamicSQLParam,
};

/// build_orm_sql
pub fn parse_orm_sql(orm_sql: &mut OrmSQL) -> BmbpResp<()> {
    let sql_build = orm_sql.get_dynamic_sql();
    let (sql, params) = match sql_build {
        crate::DynamicSQL::Query(query_sql) => {
            parse_orm_query_sql(&query_sql, orm_sql.get_dynamic_params())
        }
        crate::DynamicSQL::Insert(insert_sql) => {
            parse_orm_insert_sql(&insert_sql, orm_sql.get_dynamic_params())
        }
        crate::DynamicSQL::Update(update_sql) => {
            parse_orm_update_sql(&update_sql, orm_sql.get_dynamic_params())
        }
        crate::DynamicSQL::Delete(delete_sql) => {
            parse_orm_delete_sql(&delete_sql, orm_sql.get_dynamic_params())
        }
        crate::DynamicSQL::DDL(ddl_sql) => parse_orm_ddl_sql(ddl_sql, orm_sql.get_dynamic_params()),
        _ => Ok(("".to_string(), vec![])),
    }?;
    orm_sql.set_raw_sql(sql).set_raw_sql_params(params);
    Ok(())
}

fn parse_orm_query_sql(
    query_sql: &QuerySQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    parse_orm_query_inner(query_sql, orm_prams)
}

fn parse_orm_query_inner(
    query_sql: &QuerySQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    let select_filed = parse_orm_query_innner_field(query_sql.get_select());
    let from_table = parse_orm_query_inner_table(query_sql.get_table());
    let filter = parse_orm_query_inner_filter(query_sql.get_filter());
    let order = parse_orm_query_inner_order(query_sql.get_order());
    let group = parse_orm_query_inner_group(query_sql.get_group());

    let mut sql = "".to_string();

    if !select_filed.is_empty() {
        let select_str = select_filed.join(",");
        sql = sql + " SELECT " + select_str.as_str() + " \n ";
    }

    if !from_table.is_empty() {
        let table_str = from_table.join(",");
        sql = sql + " FROM " + table_str.as_str() + " \n ";
    }

    if !filter.is_empty() {
        sql = sql + " WHERE " + filter.as_str() + " \n ";
    }

    if !group.is_empty() {
        let group_str = group.join(",");
        sql = sql + " GROUP BY " + group_str.as_str() + " \n ";
    }

    if !order.is_empty() {
        let order_str = order.join(",");
        sql = sql + " ORDER BY " + order_str.as_str() + " \n ";
    }

    Ok((sql, vec![]))
}

fn parse_orm_query_inner_group(group: &[SelectField]) -> Vec<String> {
    vec![]
}

fn parse_orm_query_inner_order(order: &[OrderField]) -> Vec<String> {
    let mut order_vec = vec![];
    for item in order {
        order_vec.push("".to_string())
    }
    order_vec
}

fn parse_orm_query_inner_filter(filter: Option<&QueryFilter>) -> String {
    "".to_string()
}

fn parse_orm_query_innner_field(field: &[SelectField]) -> Vec<String> {
    let mut select_string_vec = vec![];

    for item in field {
        match item {
            SelectField::COLUMN(column) => {
                let field = parse_column_field_inner(column);
                select_string_vec.push(field);
            }
            SelectField::CST(cst) => {
                let field = parse_cst_field_inner(cst);
                select_string_vec.push(field);
            }
            SelectField::FUNC(func) => {}
            SelectField::EXPRESS(express) => {}
            SelectField::SQL(sql) => {}
        }
    }

    select_string_vec
}

fn parse_column_field_inner(column: &ColumnFieldInner) -> String {
    let mut column_select = "".to_string();
    let table_alias = column.table_alias();
    if !table_alias.is_empty() {
        column_select = column_select + table_alias + ".";
    }
    let field = column.field();
    column_select = column_select + field;
    let alias = column.alias();
    if !alias.is_empty() {
        column_select = column_select + " as " + alias;
    }

    let tag = column.tag();
    if tag.is_some() {
        column_select = tag.unwrap().to_string() + " " + column_select.as_str();
    }
    column_select
}

fn parse_cst_field_inner(column: &CstFieldInner) -> String {
    let field = column.filed();
    let alais = column.alias();
    let tag = column.tag();

    let mut cst_field = "".to_string();
    if tag.is_some() {
        cst_field = tag.unwrap().to_string() + cst_field.as_str();
    }

    cst_field = cst_field + field;

    if !alais.is_empty() {
        cst_field = cst_field + " AS " + alais;
    }

    cst_field
}

fn parse_orm_query_inner_table(tables: &[Table]) -> Vec<String> {
    let mut table_vec = vec![];
    for item in tables {
        let table_string = parse_table(item);
        table_vec.push(table_string);
    }
    table_vec
}

fn parse_table(table: &Table) -> String {
    let mut table_string = table.table_name().clone();

    if !table.table_alias().is_empty() {
        table_string = table_string + " AS " + table.table_alias();
    }

    if table.join().is_some() {
        let join_sql = parse_join_table(table.join().unwrap());
        table_string = table_string + "\n" + "    " + join_sql.as_str();
    }
    table_string
}

fn parse_join_table(join_table: &JoinTable) -> String {
    "".to_string()
}

fn parse_orm_insert_sql(
    insert_sql: &InsertSQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    Ok(("".to_string(), vec![]))
}

fn parse_orm_update_sql(
    update_sql: &UpdateSQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    Ok(("".to_string(), vec![]))
}

fn parse_orm_delete_sql(
    delete_sql: &DeleteSQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    Ok(("".to_string(), vec![]))
}

fn parse_orm_ddl_sql(
    ddl_sql: &DdlSQL,
    orm_prams: &DynamicSQLParam,
) -> BmbpResp<(String, Vec<Value>)> {
    Ok(("".to_string(), vec![]))
}
