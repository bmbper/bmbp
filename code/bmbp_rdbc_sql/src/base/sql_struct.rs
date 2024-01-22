use crate::{RdbcCompareType, RdbcSQLCompare, RdbcSQLCompareConcat, RdbcSQLFilter, RdbcSQLGroupBy, RdbcSQLHaving, RdbcSQLOrder, RdbcSQLSelect, RdbcSQLSet, RdbcSQLTable, RdbcSQLValue, RdbcValue};

pub enum BmbpRdbcSQLJoinType {
    INNER,
    LEFT,
    RIGHT,
    FULL,
}

pub struct BmbpRdbcSQLTable {
    schema_: Option<String>,
    alias_: Option<String>,
    name_: Box<dyn RdbcSQLTable>,
    join_: Option<BmbpRdbcSQLJoinType>,
    filter_: Option<Box<dyn RdbcSQLFilter>>,
}

impl RdbcSQLTable for BmbpRdbcSQLTable {
    fn to_table(&self) -> String {
        let mut table_name = self.name_.to_table();
        if let Some(schema) = self.schema_.as_ref() {
            table_name = format!("{}.{}", schema, table_name);
        }
        if let Some(alias) = self.alias_.as_ref() {
            table_name = format!("{} AS {}", table_name, alias);
        }
        if let Some(join) = self.join_.as_ref() {
            match join {
                BmbpRdbcSQLJoinType::INNER => {
                    table_name = format!("INNER JOIN {} ", table_name);
                }
                BmbpRdbcSQLJoinType::LEFT => {
                    table_name = format!("LEFT JOIN {} ", table_name);
                }
                BmbpRdbcSQLJoinType::RIGHT => {
                    table_name = format!("RIGHT JOIN {} ", table_name);
                }
                BmbpRdbcSQLJoinType::FULL => {
                    table_name = format!("FULL JOIN {} ", table_name);
                }
            }
        }
        if let Some(filter) = self.filter_.as_ref() {
            table_name = format!(" on {} ", filter.to_filter());
        }
        return table_name;
    }
}

impl BmbpRdbcSQLTable {
    pub fn new(name: &str) -> BmbpRdbcSQLTable {
        BmbpRdbcSQLTable {
            schema_: None,
            alias_: None,
            name_: Box::new(name.to_string()),
            join_: None,
            filter_: None,
        }
    }
}

pub struct BmbpRdbcSQLSelect {
    schema_: Option<String>,
    table_: Option<String>,
    alias_: Option<String>,
    name_: Box<dyn RdbcSQLSelect>,
}

impl RdbcSQLSelect for BmbpRdbcSQLSelect {
    fn to_select(&self) -> String {
        let mut select = self.name_.to_select();
        if let Some(table) = self.table_.as_ref() {
            let mut table_name = table.to_string();
            if let Some(schema) = self.schema_.as_ref() {
                table_name = format!("{}.{}", schema, table_name);
            }
            select = format!("{}.{}", table_name, select);
        }
        if let Some(alias) = self.alias_.as_ref() {
            select = format!("{} AS {}", select, alias);
        }
        select
    }
}

pub struct BmbpRdbcSQLInsert {
    name_: String,
    values_: RdbcValue,
}

pub struct BmbpRdbcSQLSet {
    schema_: Option<String>,
    table_: Option<String>,
    name_: String,
    value_: Option<Box<dyn RdbcSQLValue>>,
}

impl RdbcSQLSet for BmbpRdbcSQLSet {
    fn to_update(&self) -> String {
        let mut s_v = self.name_.to_string();
        if let Some(table) = self.table_.as_ref() {
            let mut table_name = table.to_string();
            if let Some(schema) = self.schema_.as_ref() {
                table_name = format!("{}.{}", schema, table_name);
            }
            s_v = format!("{}.{}", table_name, s_v);
        }
        if let Some(value) = self.value_.as_ref() {
            s_v = format!("{}={}", s_v, value.to_value())
        }
        s_v
    }
}

pub struct BmbpRdbcSQLOrder {
    name_: Box<dyn RdbcSQLSelect>,
    asc_: bool,
}

impl RdbcSQLOrder for BmbpRdbcSQLOrder {
    fn to_order(&self) -> String {
        let mut order = self.name_.to_select();
        if self.asc_ {
            order = format!("{} ASC", order);
        } else {
            order = format!("{} DESC", order);
        }
        order
    }
}

pub struct BmbpRdbcSQLGroupBy {
    name_: Box<dyn RdbcSQLSelect>,
}

impl RdbcSQLGroupBy for BmbpRdbcSQLGroupBy {
    fn to_group_by(&self) -> String {
        self.name_.to_select()
    }
}

pub struct BmbpRdbcSQLHaving {
    filter_: Option<Box<dyn RdbcSQLFilter>>,
}

impl RdbcSQLHaving for BmbpRdbcSQLHaving {
    fn to_having(&self) -> String {
        if let Some(filter) = self.filter_.as_ref() {
            return filter.to_filter();
        };
        return "".to_string();
    }
}

pub struct BmbpRdbcSQLCompare {
    schema_: Option<String>,
    table_: Option<String>,
    name_: Box<dyn RdbcSQLCompare>,
    compare_: RdbcCompareType,
    value_: Box<dyn RdbcSQLValue>,
}

impl RdbcSQLCompare for BmbpRdbcSQLCompare {
    fn to_compare(&self) -> String {
        let mut com_ = self.name_.to_compare();
        let compare_ = self.compare_.to_compare();
        let  value = self.value_.to_value();
        com_ = format!("{} {} {}", com_, compare_, value);
        if let Some(table) = self.table_.as_ref() {
            let mut table_name = table.to_string();
            if let Some(schema) = self.schema_.as_ref() {
                table_name = format!("{}.{}", schema, table_name);
            }
            com_ = format!("{}.{}", table_name, com_);
        }
        com_
    }
}

impl RdbcSQLFilter for BmbpRdbcSQLCompare {
    fn to_filter(&self) -> String {
        self.to_compare()
    }
}

pub struct BmbpRdbcSQLFilter {
    concat_: RdbcSQLCompareConcat,
    value_: Vec<Box<BmbpRdbcSQLFilter>>,
}

impl RdbcSQLFilter for BmbpRdbcSQLFilter {
    fn to_filter(&self) -> String {
        let join_ = self.concat_.to_filter();
        let mut join_value = vec![];
        for v in self.value_.iter() {
            join_value.push(v.to_filter());
        }
        join_value.join(join_.as_str())
    }
}