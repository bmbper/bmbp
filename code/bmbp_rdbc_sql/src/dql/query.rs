use crate::{RdbcFilter, RdbcOrder, RdbcSQL, RdbcColumn, RdbcTable, RdbcValue, RdbcFunc, RdbcCompareType, RdbcConcatType, RdbcTableFilterColumn, RdbcFilterColumn, RdbcTableJoinType, table};

pub struct Query {
    select_: Vec<RdbcColumn>,
    table_: Vec<RdbcTable>,
    join_: Option<Vec<RdbcTable>>,
    filter_: Option<RdbcFilter>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<RdbcFilter>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<Vec<RdbcValue>>,
}

impl RdbcSQL for Query {
    fn to_sql(&self) -> String {
        let mut sql = vec![];
        if !self.select_.is_empty() {
            let select = self.select_.iter().map(|c| c.to_sql()).collect::<Vec<_>>().join(",");
            sql.push(format!("SELECT {}", select));
        }
        sql.join(" \n")
    }
}

impl Query {
    pub fn new() -> Query {
        Query {
            select_: vec![],
            table_: vec![],
            join_: Some(vec![]),
            filter_: Some(RdbcFilter::new()),
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
}


impl Query {
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = RdbcFilter::concat_with_filter(concat, filter);
        self.filter_ = Some(new_filter);
        self
    }
    pub fn select<T>(&mut self, column: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::column(column));
        self
    }
    pub fn select_slice<T>(&mut self, columns: &[T]) -> &mut Self where T: ToString {
        for column in columns {
            self.select(column.to_string());
        }
        self
    }
    pub fn select_as_alias<T, E>(&mut self, column: T, alias: E) -> &mut Self where T: ToString, E: ToString {
        self.select_.push(RdbcColumn::column_as_alias(column, alias));
        self
    }
    pub fn select_slice_as_alias<T, E>(&mut self, columns: &[(T, E)]) -> &mut Self where T: ToString, E: ToString {
        for (column, alias) in columns {
            self.select_.push(RdbcColumn::column_as_alias(column.to_string(), alias.to_string()));
        }
        self
    }

    pub fn select_table_column<T, C>(&mut self, table: T, column: C) -> &mut Self where T: ToString, C: ToString {
        self.select_.push(RdbcColumn::table_column(table, column));
        self
    }
    pub fn select_schema_table_column<S, T, C>(&mut self, schema: S, table: T, column: C) -> &mut Self where S: ToString, T: ToString, C: ToString {
        self.select_.push(RdbcColumn::schema_table_column(schema, table, column));
        self
    }
    pub fn select_table_column_as_alias<T>(&mut self, table: T, column: T, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::table_column_as_alias(table, column, alias));
        self
    }
    pub fn select_schema_table_column_as_alias<T>(&mut self, schema: T, table: T, column: T, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::schema_table_column_as_alias(schema, table, column, alias));
        self
    }

    pub fn select_rdbc_value(&mut self, column: RdbcValue) -> &mut Self {
        self.select_.push(RdbcColumn::rdbc_value(column));
        self
    }
    pub fn select_rdbc_value_as_alias<T>(&mut self, column: RdbcValue, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::rdbc_value_alias(column, alias));
        self
    }
    pub fn select_raw_value<T>(&mut self, column: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::raw_value(column));
        self
    }
    pub fn select_raw_value_as_alias<T>(&mut self, column: T, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::raw_value_alias(column, alias));
        self
    }
    pub fn select_string_value<T>(&mut self, column: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::string_value(column));
        self
    }
    pub fn select_string_value_as_alias<T>(&mut self, column: T, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::string_value_alias(column, alias));
        self
    }

    pub fn select_query<T>(&mut self, column: Query) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::query(column));
        self
    }
    pub fn select_query_as_alias<T>(&mut self, column: Query, alias: T) -> &mut Self where T: ToString {
        self.select_.push(RdbcColumn::query_alias(column, alias));
        self
    }
    pub fn select_rdbc_func(&mut self, column: RdbcFunc) -> &mut Self {
        self
    }
    pub fn select_rdbc_func_as_alias<T>(&mut self, column: RdbcFunc, alias: T) -> &mut Self where T: ToString {
        self
    }
    pub fn select_column<T>(&mut self, column: RdbcColumn) -> &mut Self {
        self.select_.push(column);
        self
    }

    pub fn query_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table(table));
        self
    }
    pub fn query_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table(schema, table));
        self
    }
    pub fn query_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table_alias(table, alias));
        self
    }
    pub fn query_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table_alias(schema, table, alias));
        self
    }
    pub fn query_temp_table(&mut self, table: Query) -> &mut Self {
        self.table_.push(RdbcTable::temp_table(table));
        self
    }
    pub fn query_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::temp_table_alias(table, alias));
        self
    }
    pub fn query_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
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
    pub fn left_join_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
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
    pub fn right_join_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
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
    pub fn full_join_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
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
    pub fn inner_join_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
        self
    }

    pub fn order_by<T>(&mut self, column: T, is_asc: bool) -> &mut Self where T: ToString {
        self
    }

    pub fn order_asc<T>(&mut self, column: T) -> &mut Self where T: ToString {
        self
    }
    pub fn order_desc<T>(&mut self, column: T) -> &mut Self where T: ToString {
        self
    }
    pub fn order_slice<T>(&mut self, column: &[T], is_asc: bool) -> &mut Self where T: ToString {
        self
    }
    pub fn order_slice_asc<T>(&mut self, column: &[T]) -> &mut Self where T: ToString {
        self
    }
    pub fn order_slice_desc<T>(&mut self, column: &[T]) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column<T>(&mut self, column: RdbcColumn, is_asc: bool) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_vec<T>(&mut self, column: Vec<RdbcColumn>, is_asc: bool) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_slice<T>(&mut self, column: &[RdbcColumn], is_asc: bool) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_asc<T>(&mut self, column: RdbcColumn) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_vec_asc<T>(&mut self, column: Vec<RdbcColumn>) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_slice_asc<T>(&mut self, column: &[RdbcColumn]) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_desc<T>(&mut self, column: RdbcColumn) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_vec_desc<T>(&mut self, column: Vec<RdbcColumn>) -> &mut Self where T: ToString {
        self
    }
    pub fn order_column_slice_desc<T>(&mut self, column: &[RdbcColumn]) -> &mut Self where T: ToString {
        self
    }
    pub fn where_column_raw<T>(&mut self, column: RdbcColumn, compare: RdbcCompareType, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn where_column_string<T>(&mut self, column: RdbcColumn, compare: RdbcCompareType, value: Option<T>) -> &mut Self where T: ToString {
        self
    }
    pub fn where_column_rdbc_value<T>(&mut self, column: RdbcColumn, compare: RdbcCompareType, value: Option<RdbcValue>) -> &mut Self {
        self
    }
}

impl Query {
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

#[cfg(test)]
mod tests {
    use std::ptr::eq;
    use crate::{left_table, Query, RdbcSQL, RdbcTable, RdbcValue, table};

    #[test]
    fn test_select() {
        let mut query = Query::new();
        query.select("id").select("name".to_string())
            .select_as_alias("张山", "manager")
            .select_as_alias("李四".to_string(), "secondManager")
            .select_as_alias(-1i8, "payLevel")
            .select_as_alias(-2i16, "payInitStore")
            .select_as_alias(-3i32, "payPrice")
            .select_as_alias(-4i64, "payStock")
            .select_as_alias(-5i128, "paySort")
            .select_as_alias(-6isize, "paySum")
            .select_as_alias(1u8, "initLevel")
            .select_as_alias(2u16, "initStore")
            .select_as_alias(3u32, "initPrice")
            .select_as_alias(4u64, "initStock")
            .select_as_alias(5u128, "initSort")
            .select_as_alias(6usize, "initSum");
        let mut sub_query_column = Query::new();
        sub_query_column.select("sum_name").select("go_name".to_string());
        query.select_query_as_alias(sub_query_column, "subQuery");
        query.query_table("bmbp_setting_dict");
        query.eq("abc", "1");
        query.or().eq("def", "2").eq("c", "3").and().eq("c", "4");
        query.join_table("a").on().unwrap().eq("a", "b").eq("c", "d").or().eq("e", "f").eq("g", "h");
        println!("{}", query.to_sql())
    }
}