use std::collections::HashMap;
use crate::{RdbcFilterInner, RdbcOrder, RdbcSQL, RdbcColumn, RdbcTableInner, RdbcValue, RdbcConcatType, DatabaseType, RdbcFilter, RdbcTable, RdbcValueColumn};

pub struct Query {
    driver_: Option<DatabaseType>,
    select_: Vec<RdbcColumn>,
    table_: Vec<RdbcTableInner>,
    join_: Option<Vec<RdbcTableInner>>,
    filter_: Option<RdbcFilterInner>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<RdbcFilterInner>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<HashMap<String, RdbcValue>>,
}


impl Query {
    pub fn new() -> Query {
        Query {
            driver_: None,
            select_: vec![],
            table_: vec![],
            join_: Some(vec![]),
            filter_: Some(RdbcFilterInner::new()),
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
    pub fn to_params(&self) -> Vec<RdbcValue> {
        if let Some(params) = self.params_.as_ref() {
            vec![]
        } else {
            vec![]
        }
    }
    pub fn to_sql_params(&self) -> (String, Vec<RdbcValue>) {
        (self.to_sql(), self.to_params())
    }
}


impl Query {
    // RdbcColumn: From<RC>, RdbcValue: From<RV>, SS: ToString, ST: ToString, SC: ToString, SA: ToString
    pub fn select<RC>(&mut self, column: RC) -> &mut Self where RdbcColumn: From<RC> {
        self.select_.push(RdbcColumn::from(column));
        self
    }
    pub fn select_vec<RC>(&mut self, columns: Vec<RC>) -> &mut Self where RdbcColumn: From<RC> {
        for column in columns {
            self.select(column);
        }
        self
    }
    pub fn select_table_column<ST, SC>(&mut self, table: ST, column: SC) -> &mut Self where SC: ToString, ST: ToString {
        self.select_.push(RdbcColumn::table_column(table, column));
        self
    }
    pub fn select_table_column_as_alias<ST, SC, SA>(&mut self, table: ST, column: SC, alias: SA) -> &mut Self where ST: ToString, SC: ToString, SA: ToString {
        self.select_.push(RdbcColumn::table_column_as_alias(table, column, alias));
        self
    }
    pub fn select_schema_table_column<SS, ST, SC>(&mut self, schema: SS, table: ST, column: SC) -> &mut Self where SS: ToString, ST: ToString, SC: ToString {
        self.select_.push(RdbcColumn::schema_table_column(schema, table, column));
        self
    }
    pub fn select_schema_table_column_as_alias<SS, ST, SC, SA>(&mut self, schema: SS, table: ST, column: SC, alias: SA) -> &mut Self where SS: ToString, ST: ToString, SC: ToString, SA: ToString {
        self.select_.push(RdbcColumn::schema_table_column_as_alias(schema, table, column, alias));
        self
    }
    pub fn select_value<RV>(&mut self, column: RV) -> &mut Self where RdbcValue: From<RV> {
        self.select_.push(RdbcColumn::Value(RdbcValueColumn::rdbc_value(RdbcValue::from(column))));
        self
    }

    pub fn order_by<SC>(&mut self, column: SC, is_asc: bool) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_asc<SC>(&mut self, column: SC) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_desc<SC>(&mut self, column: SC) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_slice<SC>(&mut self, column: &[SC], is_asc: bool) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_slice_asc<SC>(&mut self, column: &[SC]) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_slice_desc<SC>(&mut self, column: &[SC]) -> &mut Self where SC: ToString {
        self
    }
    pub fn order_column(&mut self, column: RdbcColumn, is_asc: bool) -> &mut Self {
        self
    }
    pub fn order_column_vec(&mut self, column: Vec<RdbcColumn>, is_asc: bool) -> &mut Self {
        self
    }
    pub fn order_column_slice(&mut self, column: &[RdbcColumn], is_asc: bool) -> &mut Self {
        self
    }
    pub fn order_column_asc(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    pub fn order_column_vec_asc(&mut self, column: Vec<RdbcColumn>) -> &mut Self {
        self
    }
    pub fn order_column_slice_asc(&mut self, column: &[RdbcColumn]) -> &mut Self {
        self
    }
    pub fn order_column_desc(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    pub fn order_column_vec_desc(&mut self, column: Vec<RdbcColumn>) -> &mut Self {
        self
    }
    pub fn order_column_slice_desc(&mut self, column: &[RdbcColumn]) -> &mut Self {
        self
    }
    pub fn group_by<RC>(&mut self, column: RC) -> &mut Self where RdbcColumn: From<RC> {
        self
    }
}


impl RdbcTable for Query {}

impl RdbcFilter for Query {
    fn init_filter(&mut self) -> &mut Self {
        self
    }

    fn get_filter_mut(&mut self) -> &mut RdbcFilterInner {
        todo!()
    }

    fn add_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self {
        self
    }
}

impl RdbcSQL for Query {
    fn to_sql(&self) -> String {
        let mut sql = vec![];
        if !self.select_.is_empty() {
            let select = self.select_.iter().map(|c| c.to_sql()).collect::<Vec<_>>().join(",");
            sql.push(format!("SELECT {}", select));
        }

        if !self.table_.is_empty() {
            let table = self.table_.iter().map(|t| t.to_sql()).collect::<Vec<_>>().join(",");
            sql.push(format!("FROM {}", table));
        }
        if let Some(ref join) = self.join_ {
            let table = join.iter().map(|t| t.to_sql()).collect::<Vec<_>>().join("\n");
            sql.push(format!("{}", table));
        }
        if let Some(ref filter) = self.filter_ {
            let filter = filter.to_sql();
            if !filter.is_empty() {
                sql.push(format!("WHERE {}", filter));
            }
        }
        if let Some(ref group_by) = self.group_by_ {
            let group_by = group_by.iter().map(|c| c.to_sql()).collect::<Vec<_>>().join(",");
            sql.push(format!("GROUP BY {}", group_by));
        }
        if let Some(ref having) = self.having_ {
            let having = having.to_sql();
            sql.push(format!("HAVING {}", having));
        }
        if let Some(ref order) = self.order_ {
            let order = order.iter().map(|o| o.to_sql()).collect::<Vec<_>>().join(",");
            sql.push(format!("ORDER BY {}", order));
        }
        sql.join(" \n")
    }
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        let mut sql = self.to_sql();
        let mut sql_params = vec![];
        if let Some(ref params) = self.params_ {
            for (key, value) in params {
                sql = sql.replace(key, format!("${}", params.len()).as_str());
                sql_params.push(value.clone());
            }
        };
        (sql, sql_params)
    }
}