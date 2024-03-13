use std::collections::HashMap;
use crate::{DatabaseType, Query, RdbcColumn, RdbcConcatType, RdbcFilter, RdbcOrder, RdbcQueryFilter, RdbcSQL, RdbcTable, RdbcTableJoinType, RdbcValue, Update};

pub struct Delete {
    driver_: Option<DatabaseType>,
    table_: Vec<RdbcTable>,
    join_: Option<Vec<RdbcTable>>,
    filter_: Option<RdbcFilter>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<RdbcFilter>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcSQL for Delete {
    fn to_sql(&self) -> String {
        "".to_string()
    }

    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

impl Delete {
    pub fn new() -> Delete {
        Delete {
            driver_: None,
            table_: Vec::new(),
            join_: None,
            filter_: None,
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
    pub fn driver(driver: DatabaseType) -> Self {
        Delete {
            driver_: Some(driver),
            table_: Vec::new(),
            join_: None,
            filter_: None,
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
}

impl Delete {
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = RdbcFilter::concat_with_filter(concat, filter);
        self.filter_ = Some(new_filter);
        self
    }
    pub fn delete_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table(table));
        self
    }
    pub fn delete_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table(schema, table));
        self
    }
    pub fn delete_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table_alias(table, alias));
        self
    }
    pub fn delete_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table_alias(schema, table, alias));
        self
    }
    pub fn delete_temp_table(&mut self, table: Query) -> &mut Self {
        self.table_.push(RdbcTable::temp_table(table));
        self
    }
    pub fn delete_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::temp_table_alias(table, alias));
        self
    }
    pub fn delete_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
        self.table_.push(table);
        self
    }

    pub fn join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::table(table));
        self
    }
    pub fn on(&mut self) -> Option<&mut RdbcTable> {
        self.join_.as_mut().unwrap().get_mut(0)
    }
    pub fn on_index(&mut self, index: usize) -> Option<&mut RdbcTable> {
        self.join_.as_mut().unwrap().get_mut(index)
    }
    pub fn join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::schema_table(schema, table));
        self
    }
    pub fn join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::table_alias(table, alias));
        self
    }
    pub fn join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::schema_table_alias(schema, table, alias));
        self
    }
    pub fn join_temp_table(&mut self, table: Query) -> &mut Self {
        self.join_.as_mut().unwrap().push(RdbcTable::temp_table(table));
        self
    }
    pub fn join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::temp_table_alias(table, alias));
        self
    }
    pub fn join_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
        self.join_.as_mut().unwrap().push(table);
        self
    }
    pub fn left_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.join_.as_mut().unwrap().push(RdbcTable::left_join_table(table));
        self
    }
    pub fn left_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn left_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn left_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn left_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    pub fn left_join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn left_join_rdbc_table(&mut self, mut table: RdbcTable) -> &mut Self {
        table.join(RdbcTableJoinType::Left);
        self.join_.as_mut().unwrap().push(table);
        self
    }
    pub fn right_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn right_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn right_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn right_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn right_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    pub fn right_join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn right_join_rdbc_table(&mut self, mut table: RdbcTable) -> &mut Self {
        table.join(RdbcTableJoinType::Right);
        self.join_.as_mut().unwrap().push(table);
        self
    }

    pub fn full_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn full_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn full_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn full_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn full_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    pub fn full_join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn full_join_rdbc_table(&mut self, mut table: RdbcTable) -> &mut Self {
        table.join(RdbcTableJoinType::Full);
        self.join_.as_mut().unwrap().push(table);
        self
    }
    pub fn inner_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn inner_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    pub fn inner_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn inner_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn inner_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    pub fn inner_join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn inner_join_rdbc_table(&mut self, mut table: RdbcTable) -> &mut Self {
        table.join(RdbcTableJoinType::Inner);
        self.join_.as_mut().unwrap().push(table);
        self
    }
}

impl Delete {
    pub fn and(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::And);
        self
    }
    pub fn or(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::And);
        self
    }

    pub fn eq<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        self.filter_.as_mut().unwrap().eq(column, value);
        self
    }
    pub fn eq_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn eq_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn ne<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn ne_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn ne_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn gt<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn gt_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn gt_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn gte<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn gte_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn gte_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn lt<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn lt_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn lt_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn lte<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn lte_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn lte_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn is_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    pub fn lte_not_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }

    pub fn like<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn like_left<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_left_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_left_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn like_right<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_right_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn like_right_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn not_like<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn not_like_left<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_left_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_left_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn not_like_right<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_right_raw<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_like_right_string<T>(&mut self, column: RdbcColumn, value: Option<T>) -> &mut Self where T: ToString {
        self
    }

    pub fn in_<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }
    pub fn in_raw<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }
    pub fn in_string<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }

    pub fn in_query<T>(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }
    pub fn not_in<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_in_raw<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_in_string<T>(&mut self, column: RdbcColumn, value: Option<Vec<T>>) -> &mut Self where T: ToString {
        self
    }
    pub fn not_in_query<T>(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }

    pub fn exists_query<T>(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }
    pub fn not_exists_query<T>(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }
}
