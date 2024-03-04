use std::collections::HashMap;
use crate::{DatabaseType, RdbcColumn, RdbcFilter, RdbcDmlValue, RdbcOrder, RdbcTable, RdbcValue, RdbcTableColumn, RdbcSQL, table, Query, RdbcTableJoinType};

pub struct Update {
    driver_: Option<DatabaseType>,
    set_values_: Option<HashMap<String, RdbcDmlValue>>,
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

impl RdbcSQL for Update {
    fn to_sql(&self) -> String {
        "".to_string()
    }

    fn to_sql_with_params(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

impl Update {
    pub fn new() -> Update {
        Update {
            driver_: None,
            set_values_: None,
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
    pub fn driver(driver_: DatabaseType) -> Self {
        Update {
            driver_: Some(driver_),
            set_values_: None,
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

impl Update {
    pub fn update_table<T>(&mut self, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table(table));
        self
    }
    pub fn update_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table(schema, table));
        self
    }
    pub fn update_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::table_alias(table, alias));
        self
    }
    pub fn update_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::schema_table_alias(schema, table, alias));
        self
    }
    pub fn update_temp_table(&mut self, table: Query) -> &mut Self {
        self.table_.push(RdbcTable::temp_table(table));
        self
    }
    pub fn update_temp_table_as_alias<T>(&mut self, table: Query, alias: T) -> &mut Self where T: ToString {
        self.table_.push(RdbcTable::temp_table_alias(table, alias));
        self
    }
    pub fn update_rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
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

    pub fn set<T, V>(&mut self, column: T, value: V) -> &mut Self where T: ToString, V: ToString {
        if self.set_values_.is_none() {
            self.set_values_ = Some(HashMap::new());
        }
        let value = RdbcDmlValue::VALUE(RdbcValue::String(value.to_string()));
        self.set_values_.as_mut().unwrap().insert(column.to_string(), value);
        self
    }


    pub fn set_column<V>(&mut self, column: RdbcTableColumn, value: V) -> &mut Self where V: ToString {
        if self.set_values_.is_none() {
            self.set_values_ = Some(HashMap::new());
        }
        let value = RdbcDmlValue::VALUE(RdbcValue::String(value.to_string()));
        self.set_values_.as_mut().unwrap().insert(column.to_sql(), value);
        self
    }

    pub fn set_column_value(&mut self, column: RdbcTableColumn, value: RdbcDmlValue) -> &mut Self {
        if self.set_values_.is_none() {
            self.set_values_ = Some(HashMap::new());
        }
        self.set_values_.as_mut().unwrap().insert(column.to_sql(), value);
        self
    }
}