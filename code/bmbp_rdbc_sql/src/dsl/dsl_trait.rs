use crate::{Delete, Insert, Query, RdbcColumn, RdbcConcatType, RdbcFilterInner, RdbcFilterItem, RdbcTableInner, RdbcTableJoinType, RdbcValue, Update, value_column};


pub trait RdbcFilter {
    fn init_filter(&mut self) -> &mut Self;
    fn get_filter_mut(&mut self) -> &mut RdbcFilterInner;
    fn add_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self;
    fn and(&mut self) -> &mut Self {
        self.add_filter(RdbcConcatType::And);
        self
    }
    fn or(&mut self) -> &mut Self {
        self.add_filter(RdbcConcatType::And);
        self
    }
    fn eq_<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn eq_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn eq_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn eq_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_eq<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_eq_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_eq_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_eq_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_eq_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn ne<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn ne_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn ne_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn ne_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_ne<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_ne_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_ne_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_ne_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_ne_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn ge<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn ge_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn ge_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn ge_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_ge<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_ge_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_ge_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_ge_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_ge_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn gt<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn gt_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn gt_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn gt_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_gt<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_gt_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_gt_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_gt_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_gt_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn le<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn le_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn le_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn le_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_le<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_le_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_le_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_le_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_le_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn lt<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn lt_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn lt_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn lt_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_lt<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_lt_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_lt_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_lt_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_lt_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn like<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn like_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_like<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_like_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_like_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_like_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn like_left<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_left_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn like_left_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_left_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_like_left<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_like_left_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_like_left_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_left_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_like_left_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn like_right<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_right_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn like_right_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn like_right_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_like_right<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_like_right_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_like_right_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_right_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_like_right_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn not_like_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_not_like<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_not_like_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_not_like_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_not_like_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like_left<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_left_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn not_like_left_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_left_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_not_like_left<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_not_like_left_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_not_like_left_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_left_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_not_like_left_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like_right<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_right_col<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcColumn> {
        self
    }
    fn not_like_right_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_like_right_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_not_like_right<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_not_like_right_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: Into<RdbcColumn> {
        self
    }
    fn col_not_like_right_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_right_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn col_not_like_right_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn in_v<T, V>(&mut self, column: T, value: Vec<V>) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn in_v_slice<T, V>(&mut self, column: T, value: &[V]) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn in_query<T, V>(&mut self, column: T, value: Query) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn in_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_in_v<V>(&mut self, column: RdbcColumn, value: Vec<V>) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_in_slice<V>(&mut self, column: RdbcColumn, value: &[V]) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_in_query(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }
    fn col_in_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }

    fn not_in_v<T, V>(&mut self, column: T, value: Vec<V>) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_in_v_slice<T, V>(&mut self, column: T, value: &[V]) -> &mut Self where T: Into<RdbcColumn>, V: Into<RdbcValue> {
        self
    }
    fn not_in_query<T, V>(&mut self, column: T, value: Query) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn not_in_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, V: ToString {
        self
    }
    fn col_not_in_v<V>(&mut self, column: RdbcColumn, value: Vec<V>) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_not_in_slice<V>(&mut self, column: RdbcColumn, value: &[V]) -> &mut Self where V: Into<RdbcValue> {
        self
    }
    fn col_not_in_query(&mut self, column: RdbcColumn, value: Query) -> &mut Self {
        self
    }
    fn col_not_in_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self where V: ToString {
        self
    }
    fn is_null<T>(&mut self, column: T) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn col_is_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    fn not_null<T>(&mut self, column: T) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn col_not_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }

    fn exists_<T>(&mut self, column: T, value: Query) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn exists_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, T: ToString {
        self
    }
    fn not_exists_<T>(&mut self, column: T, value: Query) -> &mut Self where T: Into<RdbcColumn> {
        self
    }
    fn not_exists_raw<T, V>(&mut self, column: T, value: V) -> &mut Self where T: Into<RdbcColumn>, T: ToString {
        self
    }
}
pub trait RdbcTable {
    fn table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn table_alias<T, V>(&mut self, table: T, alias: V) -> &mut Self where T: ToString, V: ToString {
        self
    }
    fn schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    fn schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn temp_table_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn rdbc_table<T>(&mut self, table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }

    fn on(&mut self) -> Option<&mut RdbcTableInner> {
        None
    }
    fn on_index(&mut self, index: usize) -> Option<&mut RdbcTableInner> {
        None
    }
    fn join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    fn join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn join_temp_table_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn join_rdbc_table<T>(&mut self, table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }
    fn left_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn left_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn left_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }

    fn left_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn left_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn left_join_temp_table_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn left_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }

    fn right_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn right_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn right_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    fn right_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn right_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn right_join_temp_table_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn right_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }

    fn full_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn full_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn full_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    fn full_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn full_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn full_join_temp_table_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn full_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }
    fn inner_join_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self
    }
    fn inner_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn inner_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self
    }
    fn inner_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn inner_join_temp_table(&mut self, table: Query) -> &mut Self {
        self
    }
    fn inner_join_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self
    }
    fn inner_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self where T: Into<RdbcTableInner> {
        self
    }
}



pub trait RdbcSQL {
    fn to_sql(&self) -> String;
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

impl<T> RdbcSQL for T where T: ToString {
    fn to_sql(&self) -> String {
        self.to_string()
    }
    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        (self.to_string(), vec![])
    }
}

/// RdbcSQLParser 语句解析器
pub trait RdbcSQLParser {
    fn to_query(&self, query: &Query) -> (String, Vec<RdbcValue>);
    fn to_insert(&self, query: &Insert) -> (String, Vec<RdbcValue>);
    fn to_update(&self, query: &Update) -> (String, Vec<RdbcValue>);
    fn to_delete(&self, query: &Delete) -> (String, Vec<RdbcValue>);
}


