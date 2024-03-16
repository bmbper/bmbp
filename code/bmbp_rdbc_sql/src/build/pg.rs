use std::collections::HashMap;

use crate::build::base::base_build_sql;
use crate::build::vars::PG_PARAMS_TAG;
use crate::{
    DatabaseType, Delete, Insert, Query, RdbcColumn, RdbcConcatFunc, RdbcFunc, RdbcFuncColumn,
    RdbcQueryColumn, RdbcSQL, RdbcTableColumn, RdbcValue, RdbcValueColumn, Update,
};

pub fn pg_build_sql(sql: String, params: HashMap<String, RdbcValue>) -> (String, Vec<RdbcValue>) {
    base_build_sql(PG_PARAMS_TAG, sql, params)
}
pub fn pg_build_query_script(query: &Query) -> (String, HashMap<String, RdbcValue>) {
    let mut sql = "".to_string();
    let mut sql_prams = HashMap::new();
    if !query.get_select().is_empty() {
        let (select_sql, select_params) = pg_build_select_sql(query.get_select());
        sql = format!("SELECT {} ", select_sql.join(","));
        sql_prams.extend(select_params.into_iter());
    }
    (sql, sql_prams)
}

fn pg_build_select_sql(
    select_columns: &Vec<RdbcColumn>,
) -> (Vec<String>, HashMap<String, RdbcValue>) {
    let (mut select_sql, mut select_params) = (vec![], HashMap::new());
    for column in select_columns {
        let (column_sql, column_params) = pg_build_select_column_sql(column);
        select_sql.push(column_sql);
        select_params.extend(column_params.into_iter());
    }
    (select_sql, select_params)
}

fn pg_build_select_column_sql(column: &RdbcColumn) -> (String, HashMap<String, RdbcValue>) {
    match column {
        RdbcColumn::Table(tc) => pg_build_select_table_column_sql(tc),
        RdbcColumn::Query(qc) => pg_build_select_query_column_sql(qc),
        RdbcColumn::Func(fc) => pg_build_select_func_column_sql(fc),
        RdbcColumn::Value(vc) => pg_build_select_value_column_sql(vc),
    }
}

fn pg_build_select_value_column_sql(
    column: &RdbcValueColumn,
) -> (String, HashMap<String, RdbcValue>) {
    let mut sql = "".to_string();
    let value = column.get_name();
    sql = match value {
        RdbcValue::String(value) => format!("'{}'", value),
        RdbcValue::Int(value) => value.to_string(),
        RdbcValue::Float(value) => value.to_string(),
        RdbcValue::Bool(value) => value.to_string(),
        RdbcValue::DateTime(value) => format!("'{}'", value),
        RdbcValue::Null => "null".to_string(),
        RdbcValue::BigInt(v) => value.to_string(),
        RdbcValue::BigFloat(v) => value.to_string(),
    };

    if let Some(alias) = column.get_alias() {
        sql = format!("{} AS {}", sql, alias);
    }
    (sql, HashMap::new())
}

fn pg_build_select_func_column_sql(
    column: &RdbcFuncColumn,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut sql, mut params) = ("".to_string(), HashMap::new());
    match column.get_name() {
        RdbcFunc::CONCAT(concat) => {
            let (func_sql, func_params) = pg_build_func_concat_sql(concat);
            sql = func_sql;
            params.extend(func_params.into_iter())
        }
    };

    if let Some(alias) = column.get_alias() {
        sql = format!("{} AS {}", sql, alias);
    }
    (sql, params)
}

fn pg_build_select_query_column_sql(
    column: &RdbcQueryColumn,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut sql, mut params) = ("".to_string(), HashMap::new());
    let (query_sql, query_params) = column.get_name().build_script(DatabaseType::Postgres);
    sql = query_sql;
    params.extend(query_params.into_iter());
    if let Some(alias) = column.get_alias() {
        sql = format!("{} AS {}", sql, alias);
    }
    (sql, params)
}

fn pg_build_select_table_column_sql(
    column: &RdbcTableColumn,
) -> (String, HashMap<String, RdbcValue>) {
    let mut sql = column.get_name().clone();
    if let Some(table) = column.get_table() {
        sql = format!("{}.{}", table, sql);
        if let Some(schema) = column.get_schema() {
            sql = format!("{}.{}", schema, sql);
        }
    }
    if let Some(alias) = column.get_alias() {
        sql = format!("{} AS {}", sql, alias);
    }
    (sql, HashMap::new())
}

pub fn pg_build_insert_script(insert: &Insert) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn pg_build_update_script(update: &Update) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn pg_build_delete_script(delete: &Delete) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}

fn pg_build_func_concat_sql(func: &RdbcConcatFunc) -> (String, HashMap<String, RdbcValue>) {
    let (mut func_sql, mut func_params) = ("".to_string(), HashMap::new());
    let mut column_vec = vec![];
    for column in func.get_columns() {
        let (column_sql, column_params) = pg_build_select_column_sql(column);
        column_vec.push(column_sql);
        func_params.extend(column_params.into_iter());
    }
    if let Some(liter) = func.get_liter() {
        func_sql = format!(
            "CONCAT({})",
            column_vec.join(format!(",'{}',", liter).as_str())
        );
    } else {
        func_sql = format!("CONCAT({})", column_vec.join(","));
    }
    (func_sql, func_params)
}
