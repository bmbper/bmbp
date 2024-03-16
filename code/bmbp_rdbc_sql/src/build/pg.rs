use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    DatabaseType, Delete, Insert, Query, RdbcColumn, RdbcColumnFilterItem, RdbcCompareType,
    RdbcConcatFunc, RdbcConcatType, RdbcFilterInner, RdbcFilterItem, RdbcFunc, RdbcFuncColumn,
    RdbcOrder, RdbcOrderType, RdbcQueryColumn, RdbcQueryTable, RdbcSchemaTable, RdbcSQL,
    RdbcTableColumn, RdbcTableInner, RdbcTableJoinType, RdbcValue, RdbcValueColumn,
    RdbcValueFilterItem, Update,
};
use crate::build::base::base_build_sql;
use crate::build::vars::PG_PARAMS_TAG;

pub fn pg_build_sql(sql: String, params: HashMap<String, RdbcValue>) -> (String, Vec<RdbcValue>) {
    base_build_sql(PG_PARAMS_TAG, sql, params)
}
pub fn pg_build_query_script(query: &Query) -> (String, HashMap<String, RdbcValue>) {
    let mut sql = "".to_string();
    let mut sql_prams = HashMap::new();

    // 解析返回值
    if !query.get_select().is_empty() {
        let (select_sql, select_params) = pg_build_select_sql(query.get_select(), true);
        sql = format!("SELECT {} ", select_sql.join(","));
        sql_prams.extend(select_params.into_iter());
    }

    // 解析表结构
    if !query.get_table().is_empty() {
        let (table_sql, table_params) = pg_build_table_vec_sql(query.get_table());
        sql = format!("{} FROM {} ", sql, table_sql.join(","));
        sql_prams.extend(table_params.into_iter());
    }
    // 关联表
    if query.get_join().is_some() {
        let (join_sql, join_params) = pg_build_table_vec_sql(query.get_join().unwrap());
        sql = format!("{}  {} ", sql, join_sql.join("\n"));
        sql_prams.extend(join_params.into_iter());
    }
    // 查询条件
    if query.get_filter().is_some() {
        let (filter_sql, filter_params) = pg_build_filter_sql(query.get_filter());
        if !filter_sql.is_empty() {
            sql = format!("{} WHERE {} ", sql, filter_sql);
            sql_prams.extend(filter_params.into_iter());
        }
    }
    // 分组
    if query.get_group_by().is_some() {
        let (group_by_sql, group_by_params) = pg_build_group_by(query.get_group_by().unwrap());
        if !group_by_sql.is_empty() {
            sql = format!(" {} GROUP BY {} ", sql, group_by_sql.join(","));
            sql_prams.extend(group_by_params.into_iter());
        }
    }
    // 分组条件
    if query.get_having().is_some() {
        let (having_sql, having_params) = pg_build_filter_sql(query.get_having());
        if !having_sql.is_empty() {
            sql = format!(" {} HAVING {} ", sql, having_sql);
            sql_prams.extend(having_params.into_iter());
        }
    }
    // 排序条件
    if query.get_order().is_some() {
        let order_sql = pg_build_order(query.get_order().unwrap());
        if !order_sql.is_empty() {
            sql = format!("{} ORDER BY {} ", sql, order_sql.join(","));
        }
    }

    // 分页
    if query.get_limit().is_some() {
        let limit = query.get_limit().unwrap();
        sql = format!("{} LIMIT {} ", sql, limit);
    }
    if query.get_offset().is_some() {
        let offset = query.get_offset().unwrap();
        sql = format!("{} OFFSET {} ", sql, offset);
    }
    // 联合语句
    if query.get_union_all().is_some() {
        for item in query.get_union_all().unwrap() {
            let (union_all_sql, union_all_params) = pg_build_query_script(item);
            sql = format!("{} UNION ALL {}", sql, union_all_sql);
            sql_prams.extend(union_all_params.into_iter());
        }
    }
    // 联合语句
    if query.get_union_only().is_some() {
        for item in query.get_union_only().unwrap() {
            let (union_only_sql, union_only_params) = pg_build_query_script(item);
            sql = format!("{} UNION ONLY {}", sql, union_only_sql);
            sql_prams.extend(union_only_params.into_iter());
        }
    }

    (sql, sql_prams)
}

fn pg_build_order(order_column: &Vec<RdbcOrder>) -> Vec<String> {
    let mut order_vec = vec![];
    for order_ in order_column {
        order_vec.push(pg_build_order_column(order_));
    }
    order_vec
}

fn pg_build_order_column(order_: &RdbcOrder) -> String {
    match order_ {
        RdbcOrder::Column(column) => {
            let (column_sql, column_params) =
                pg_build_select_column_sql(column.get_column(), false);
            match column.get_order() {
                RdbcOrderType::Asc => format!("{} ASC", column_sql),
                RdbcOrderType::Desc => format!("{} DESC", column_sql),
            }
        }
    }
}

fn pg_build_group_by(columns: &Vec<RdbcColumn>) -> (Vec<String>, HashMap<String, RdbcValue>) {
    pg_build_select_sql(columns, false)
}

fn pg_build_table_vec_sql(
    tables: &Vec<RdbcTableInner>,
) -> (Vec<String>, HashMap<String, RdbcValue>) {
    let (mut table_sql_vec, mut table_params) = (vec![], HashMap::new());
    for table in tables {
        let (temp_sql, temp_params) = pg_build_table_sql(table);
        table_sql_vec.push(temp_sql);
        table_params.extend(temp_params.into_iter());
    }
    (table_sql_vec, table_params)
}

fn pg_build_table_sql(table: &RdbcTableInner) -> (String, HashMap<String, RdbcValue>) {
    match table {
        RdbcTableInner::Table(tb) => pg_build_table_schema_table_sql(tb),
        RdbcTableInner::Query(qb) => pg_build_table_query_table_sql(qb),
    }
}

fn pg_build_table_query_table_sql(table: &RdbcQueryTable) -> (String, HashMap<String, RdbcValue>) {
    let (mut table_sql, mut table_params) = ("".to_string(), HashMap::new());

    let query = table.get_name();
    let (query_sql, query_params) = query.build_script(DatabaseType::Postgres);

    table_sql = format!("({})", query_sql);
    table_params.extend(query_params.into_iter());

    if let Some(alias) = table.get_alias() {
        table_sql = format!(" {} AS {} ", table_sql, alias);
    }

    table_sql = pg_build_join_type(table.get_join(), table_sql);

    let (filter_sql, filter_params) = pg_build_filter_sql(table.get_filter());
    if !filter_sql.is_empty() {
        table_sql = format!(" {} ON {} ", table_sql, filter_sql);
        table_params.extend(filter_params.into_iter());
    }

    (table_sql, table_params)
}

fn pg_build_filter_sql(filter: Option<&RdbcFilterInner>) -> (String, HashMap<String, RdbcValue>) {
    let (mut filter_sql, mut filter_params) = ("".to_string(), HashMap::new());
    if let Some(rdbc_filter) = filter {
        let (item_vec, item_params) = pg_build_filter_items_sql(rdbc_filter.get_item());
        filter_params.extend(item_params.into_iter());
        match rdbc_filter.get_concat() {
            RdbcConcatType::And => {
                filter_sql = format!(" {} ", item_vec.join(" AND "));
            }
            RdbcConcatType::Or => {
                filter_sql = format!(" {} ", item_vec.join(" OR "));
            }
        }
    }

    (filter_sql, filter_params)
}

fn pg_build_filter_items_sql(
    filter_items: &Vec<RdbcFilterItem>,
) -> (Vec<String>, HashMap<String, RdbcValue>) {
    let (mut items_sql, mut items_params) = (vec![], HashMap::new());
    for item in filter_items {
        let (item_sql, item_params) = pg_build_filter_item_sql(item);
        if !item_sql.is_empty() {
            items_sql.push(item_sql);
            items_params.extend(item_params.into_iter());
        }
    }
    (items_sql, items_params)
}

fn pg_build_filter_item_sql(filter_item: &RdbcFilterItem) -> (String, HashMap<String, RdbcValue>) {
    match filter_item {
        RdbcFilterItem::Value(value) => pg_build_filter_value_sql(value),
        RdbcFilterItem::Column(column) => pg_build_filter_column_sql(column),
        RdbcFilterItem::Filter(filter) => {
            let (item_sql, item_params) = pg_build_filter_sql(Some(filter));
            (format!("({})", item_sql), item_params)
        }
    }
}

fn pg_build_filter_column_sql(
    column: &RdbcColumnFilterItem,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut item_sql, mut item_params) = ("".to_string(), HashMap::new());
    let column_left = column.get_column();
    let column_right = column.get_value();
    let column_key = Uuid::new_v4().to_string();
    let (left_column_sql, left_column_params) = pg_build_select_column_sql(column_left, false);
    let compare = column.get_compare();
    match compare {
        RdbcCompareType::IsNull => {
            item_sql = format!(" {} IS NULL  ", left_column_sql);
        }
        RdbcCompareType::IsNotNull => {
            item_sql = format!(" {} IS NOT NULL  ", left_column_sql);
        }
        RdbcCompareType::Eq => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} = {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::NotEq => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} != {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::Gt => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} > {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::GtEq => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} >= {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::Lt => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} < {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::LtEq => {
            if let Some(value) = column_right {
                let (right_column_sql, right_column_params) =
                    pg_build_select_column_sql(value, false);
                item_sql = format!(" {} <= {} ", left_column_sql, right_column_sql);
                item_params.extend(
                    left_column_params
                        .into_iter()
                        .chain(right_column_params.into_iter()),
                );
            }
        }
        RdbcCompareType::Like => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} LIKE '%${{{}}}%'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::LikeLeft => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} LIKE '${{{}}}%'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::LikeRight => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} LIKE '%${{{}}}'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::NotLike => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} NOT LIKE '%${{{}}}%'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::NotLikeLeft => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} NOT LIKE '${{{}}}%'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::NotLikeRight => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} NOT LIKE '%${{{}}}'", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
            }
        }
        RdbcCompareType::In => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} IN (${{{}}})", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
                if value.is_query() {
                    let (right_column_sql, right_column_params) =
                        pg_build_query_script(value.get_query().unwrap());
                    item_sql = format!(" {} IN ({})", left_column_sql, right_column_sql);
                    item_params.extend(right_column_params.into_iter());
                }
            }
        }
        RdbcCompareType::NotIn => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} NOT IN (${{{}}})", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
                if value.is_query() {
                    let (right_column_sql, right_column_params) =
                        pg_build_query_script(value.get_query().unwrap());
                    item_sql = format!(" {} NOT IN ({})", left_column_sql, right_column_sql);
                    item_params.extend(right_column_params.into_iter());
                }
            }
        }
        RdbcCompareType::Exists => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} EXISTS (${{{}}}) ", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
                if value.is_query() {
                    let (right_column_sql, right_column_params) =
                        pg_build_query_script(value.get_query().unwrap());
                    item_sql = format!(" {} EXISTS ({})", left_column_sql, right_column_sql);
                    item_params.extend(right_column_params.into_iter());
                }
            }
        }
        RdbcCompareType::NotExists => {
            if let Some(value) = column_right {
                if value.is_value() {
                    item_sql = format!(" {} NOT EXISTS (${{{}}}) ", left_column_sql, column_key);
                    item_params.insert(column_key.clone(), value.get_value().unwrap().clone());
                }
                if value.is_query() {
                    let (right_column_sql, right_column_params) =
                        pg_build_query_script(value.get_query().unwrap());
                    item_sql = format!(" {} NOT EXISTS ({})", left_column_sql, right_column_sql);
                    item_params.extend(right_column_params.into_iter());
                }
            }
        }
    }

    (item_sql, item_params)
}

fn pg_build_filter_value_sql(value: &RdbcValueFilterItem) -> (String, HashMap<String, RdbcValue>) {
    let (mut item_sql, mut item_params) = ("".to_string(), HashMap::new());
    let column = value.get_column();
    let (column_sql, column_params) = pg_build_select_column_sql(column, false);
    let column_key = Uuid::new_v4().to_string();
    let column_value = value.get_value();
    let ignore_null = value.get_ignore_null();
    let compare = value.get_compare();
    match compare {
        RdbcCompareType::Eq => {
            if let Some(value) = column_value {
                item_sql = format!(" {} = ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} = '' ", column_sql);
                }
            }
        }
        RdbcCompareType::NotEq => {
            if let Some(value) = column_value {
                item_sql = format!(" {} != ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} = '' ", column_sql);
                }
            }
        }
        RdbcCompareType::Gt => {
            if let Some(value) = column_value {
                item_sql = format!(" {} > ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} > '' ", column_sql);
                }
            }
        }
        RdbcCompareType::GtEq => {
            if let Some(value) = column_value {
                item_sql = format!(" {} >= ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} >= '' ", column_sql);
                }
            }
        }
        RdbcCompareType::Lt => {
            if let Some(value) = column_value {
                item_sql = format!(" {} < ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} < '' ", column_sql);
                }
            }
        }
        RdbcCompareType::LtEq => {
            if let Some(value) = column_value {
                item_sql = format!(" {} <= ${{{}}} ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} <= '' ", column_sql);
                }
            }
        }
        RdbcCompareType::Like => {
            if let Some(value) = column_value {
                item_sql = format!(" {} LIKE '%${{{}}}%'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} LIKE '%%' ", column_sql);
                }
            }
        }
        RdbcCompareType::LikeLeft => {
            if let Some(value) = column_value {
                item_sql = format!(" {} LIKE '${{{}}}%'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::LikeRight => {
            if let Some(value) = column_value {
                item_sql = format!(" {} LIKE '$%{{{}}}'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::NotLike => {
            if let Some(value) = column_value {
                item_sql = format!(" {} NOT LIKE '%${{{}}}%'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            } else {
                if !ignore_null {
                    item_sql = format!(" {} NOT LIKE '%%' ", column_sql);
                }
            }
        }
        RdbcCompareType::NotLikeLeft => {
            if let Some(value) = column_value {
                item_sql = format!(" {} NOT LIKE '${{{}}}%'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::NotLikeRight => {
            if let Some(value) = column_value {
                item_sql = format!(" {} NOT LIKE '${{{}}}%'", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::In => {
            if let Some(value) = column_value {
                item_sql = format!(" {} IN (${{{}}}) ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::NotIn => {
            if let Some(value) = column_value {
                item_sql = format!(" {} NOT IN (${{{}}}) ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::IsNull => {
            item_sql = format!(" {} IS NULL  ", column_sql);
        }
        RdbcCompareType::IsNotNull => {
            item_sql = format!(" {} IS NOT NULL  ", column_sql);
        }
        RdbcCompareType::Exists => {
            if let Some(value) = column_value {
                item_sql = format!(" {} EXISTS (${{{}}}) ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
        RdbcCompareType::NotExists => {
            if let Some(value) = column_value {
                item_sql = format!(" {} NOT EXISTS (${{{}}}) ", column_sql, column_key);
                item_params.insert(column_key.clone(), value.clone());
            }
        }
    }
    (item_sql, item_params)
}

fn pg_build_table_schema_table_sql(
    table: &RdbcSchemaTable,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut table_sql, mut table_params) = ("".to_string(), HashMap::new());

    table_sql = format!("{} ", table.get_name());
    if let Some(schema) = table.get_schema() {
        table_sql = format!("{}.{}", schema, table_sql);
    }

    if let Some(alias) = table.get_alias() {
        table_sql = format!(" {} AS {} ", table_sql, alias);
    }

    table_sql = pg_build_join_type(table.get_join(), table_sql);

    let (filter_sql, filter_params) = pg_build_filter_sql(table.get_filter());
    if !filter_sql.is_empty() {
        table_sql = format!(" {} ON {} ", table_sql, filter_sql);
        table_params.extend(filter_params.into_iter());
    }

    (table_sql, table_params)
}

fn pg_build_join_type(join_op: Option<&RdbcTableJoinType>, mut table_sql: String) -> String {
    if let Some(join_type) = join_op {
        match join_type {
            RdbcTableJoinType::Inner => {
                table_sql = format!(" INNER JOIN {} ", table_sql);
            }
            RdbcTableJoinType::Left => {
                table_sql = format!(" LEFT JOIN {}", table_sql);
            }
            RdbcTableJoinType::Right => {
                table_sql = format!(" RIGHT JOIN {}", table_sql);
            }
            RdbcTableJoinType::Full => {
                table_sql = format!(" FULL JOIN {}", table_sql);
            }
        }
    }
    table_sql
}

fn pg_build_select_sql(
    select_columns: &Vec<RdbcColumn>,
    with_alias: bool,
) -> (Vec<String>, HashMap<String, RdbcValue>) {
    let (mut select_sql, mut select_params) = (vec![], HashMap::new());
    for column in select_columns {
        let (column_sql, column_params) = pg_build_select_column_sql(column, with_alias);
        select_sql.push(column_sql);
        select_params.extend(column_params.into_iter());
    }
    (select_sql, select_params)
}

fn pg_build_select_column_sql(
    column: &RdbcColumn,
    with_alias: bool,
) -> (String, HashMap<String, RdbcValue>) {
    match column {
        RdbcColumn::Table(tc) => pg_build_select_table_column_sql(tc, with_alias),
        RdbcColumn::Query(qc) => pg_build_select_query_column_sql(qc, with_alias),
        RdbcColumn::Func(fc) => pg_build_select_func_column_sql(fc, with_alias),
        RdbcColumn::Value(vc) => pg_build_select_value_column_sql(vc, with_alias),
    }
}

fn pg_build_select_value_column_sql(
    column: &RdbcValueColumn,
    with_alias: bool,
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
    if with_alias {
        if let Some(alias) = column.get_alias() {
            sql = format!("{} AS {}", sql, alias);
        }
    }

    (sql, HashMap::new())
}

fn pg_build_select_func_column_sql(
    column: &RdbcFuncColumn,
    with_alias: bool,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut sql, mut params) = ("".to_string(), HashMap::new());
    match column.get_name() {
        RdbcFunc::CONCAT(concat) => {
            let (func_sql, func_params) = pg_build_func_concat_sql(concat, with_alias);
            sql = func_sql;
            params.extend(func_params.into_iter())
        }
    };

    if with_alias {
        if let Some(alias) = column.get_alias() {
            sql = format!("{} AS {}", sql, alias);
        }
    }
    (sql, params)
}

fn pg_build_select_query_column_sql(
    column: &RdbcQueryColumn,
    with_alias: bool,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut sql, mut params) = ("".to_string(), HashMap::new());
    let (query_sql, query_params) = column.get_name().build_script(DatabaseType::Postgres);
    sql = query_sql;
    params.extend(query_params.into_iter());
    if with_alias {
        if let Some(alias) = column.get_alias() {
            sql = format!("{} AS {}", sql, alias);
        }
    }
    (sql, params)
}

fn pg_build_select_table_column_sql(
    column: &RdbcTableColumn,
    with_alias: bool,
) -> (String, HashMap<String, RdbcValue>) {
    let mut sql = column.get_name().clone();
    if let Some(table) = column.get_table() {
        sql = format!("{}.{}", table, sql);
        if let Some(schema) = column.get_schema() {
            sql = format!("{}.{}", schema, sql);
        }
    }
    if with_alias {
        if let Some(alias) = column.get_alias() {
            sql = format!("{} AS {}", sql, alias);
        }
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

fn pg_build_func_concat_sql(
    func: &RdbcConcatFunc,
    with_alias: bool,
) -> (String, HashMap<String, RdbcValue>) {
    let (mut func_sql, mut func_params) = ("".to_string(), HashMap::new());
    let mut column_vec = vec![];
    for column in func.get_columns() {
        let (column_sql, column_params) = pg_build_select_column_sql(column, with_alias);
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
