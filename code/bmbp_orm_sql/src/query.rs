/// 查询构建器
/// 暂时支持PG，后续适配MYSQL\SQLite\SQLServer等

/// FilterConditionType  过滤条件类型
#[derive(Clone, Default)]
pub enum FilterConditionType {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    Like,
    NotLike,
    LikeLeft,
    LikeRight,
    NotLikeLeft,
    NotLikeRight,
    In_,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
    NotBetween,
    Not,
    Exists,
    NotExists,
}
impl Default for FilterConditionType {
    fn default() -> Self {
        FilterConditionType::Eq
    }
}
impl FilterConditionType {
    pub fn value(&self) -> &'static str {
        match self {
            FilterConditionType::Eq => "=",
            FilterConditionType::Ne => "!=",
            FilterConditionType::Gt => ">",
            FilterConditionType::Ge => ">=",
            FilterConditionType::Lt => "<",
            FilterConditionType::Le => "<=",
            FilterConditionType::Like => "LIKE",
            FilterConditionType::NotLike => "NOT LIKE",
            FilterConditionType::LikeLeft => "LIKE",
            FilterConditionType::LikeRight => "LIKE",
            FilterConditionType::NotLikeLeft => "NOT LIKE",
            FilterConditionType::NotLikeRight => "NOT LIKE",
            FilterConditionType::In_ => "IN",
            FilterConditionType::NotIn => "NOT IN",
            FilterConditionType::IsNull => "IS NULL",
            FilterConditionType::IsNotNull => "IS NOT NULL",
            FilterConditionType::Between => "BETWEEN",
            FilterConditionType::NotBetween => "NOT BETWEEN",
            FilterConditionType::Not => "NOT",
            FilterConditionType::Exists => "EXISTS",
            FilterConditionType::NotExists => "NOT EXISTS",
        }
    }
}

#[derive(Clone, Default)]
pub enum FilterConnectType {
    Or,
    And,
}
impl Default for FilterConnectType {
    fn default() -> Self {
        FilterConnectType::And
    }
}
/// NestFilterWrapper 嵌套过滤器包装器
#[derive(Clone, Default)]
pub struct NestFilterWrapper<'a> {
    where_filter: Vec<&'a str>,
    join_type: FilterConnectType,
}
impl<'a> NestFilterWrapper<'a> {
    pub fn new() -> NestFilterWrapper<'static> {
        NestFilterWrapper {
            where_filter: vec![],
            join_type: FilterConnectType::And,
        }
    }
    pub fn or(&mut self) -> &mut Self {
        self.join_type = FilterConnectType::Or;
        self
    }
    pub fn and(&mut self) -> &mut Self {
        self.join_type = FilterConnectType::And;
        self
    }
    pub fn or_nest(&mut self) -> NestFilterWrapper<'a> {
        NestFilterWrapper {
            where_filter: vec![],
            join_type: FilterConnectType::Or,
        }
    }
    pub fn and_nest(&mut self) -> NestFilterWrapper<'a> {
        NestFilterWrapper {
            where_filter: vec![],
            join_type: FilterConnectType::And,
        }
    }
    pub fn end(&mut self) -> &mut Self {
        self.where_filter.push(")");
        self
    }

    pub fn add_filter(&mut self, column: &'a str, typ: FilterConditionType, value: &'a str) -> &mut Self {
        self.where_filter.push(format!("{} {} {}", column, typ.value(), value).as_str());
        self
    }
}


///  QueryWrapper 查询包装器
#[derive(Clone, Default)]
pub struct QueryWrapper<'a> {
    selected_field: Vec<&'a str>,
    from_table: Vec<&'a str>,
    join_table: Vec<&'a str>,
    where_filter: Vec<&'a str>,
}


impl QueryWrapper<'_> {
    pub fn new() -> QueryWrapper<'static> {
        QueryWrapper {
            selected_field: vec![],
            from_table: vec![],
            join_table: vec![],
            where_filter: vec![],
        }
    }
    pub fn query_sql(&self) -> String {
        "".to_string()
    }
}

impl QueryWrapper<'_> {
    pub fn select(&mut self, field: &str) -> &mut Self {
        self.selected_field.push(field);
        self
    }
    pub fn select_from_slice(&mut self, fields: &[&str]) -> &mut Self {
        for field in fields {
            self.selected_field.push(field);
        }
        self
    }
    pub fn select_alias(&mut self, field: &str, alias: &str) -> &mut Self {
        self.selected_field.push(format!("{} as {}", field, alias).as_str());
        self
    }
    pub fn select_alias_from_slice(&mut self, fields: &[(&str, &str)]) {
        for (field, alias) in fields {
            self.selected_field.push(format!("{} as {}", field, alias).as_str());
        }
    }
    pub fn from(&mut self, table: &str) -> &mut Self {
        self.from_table.push(table);
        self
    }
    pub fn from_slice(&mut self, tables: &[&str]) -> &mut Self {
        for table in tables {
            self.from_table.push(table);
        }
        self
    }
    pub fn from_alias(&mut self, table: &str, alias: &str) -> &mut Self {
        self.from_table.push(format!("{} as {}", table, alias).as_str());
        self
    }
    pub fn from_alias_slice(&mut self, tables: &[(&str, &str)]) {
        for (table, alias) in tables {
            self.from_table.push(format!("{} as {}", table, alias).as_str());
        }
    }

    pub fn eq(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} = {}", field, value).as_str());
        self
    }
    pub fn ne(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} <> {}", field, value).as_str());
        self
    }
    pub fn gt(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} > {}", field, value).as_str());
        self
    }
    pub fn ge(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} >= {}", field, value).as_str());
        self
    }
    pub fn lt(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} < {}", field, value).as_str());
        self
    }
    pub fn le(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} <= {}", field, value).as_str());
        self
    }
    pub fn like(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} like {}", field, value).as_str());
        self
    }
    pub fn not_like(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} not like {}", field, value).as_str());
        self
    }
    pub fn in_(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} in {}", field, value).as_str());
        self
    }
    pub fn not_in(&mut self, field: &str, value: &str) -> &mut Self {
        self.where_filter.push(format!("{} not in {}", field, value).as_str());
        self
    }
    pub fn between(&mut self, field: &str, value1: &str, value2: &str) -> &mut Self {
        self.where_filter.push(format!("{} between {} and {}", field, value1, value2).as_str());
        self
    }
    pub fn not_between(&mut self, field: &str, value1: &str, value2: &str) -> &mut Self {
        self.where_filter.push(format!("{} not between {} and {}", field, value1, value2).as_str());
        self
    }
    pub fn and(&mut self, filter: &str) -> &mut Self {
        self.where_filter.push(filter);
        self
    }
    pub fn or(&mut self, filter: &str) -> &mut Self {
        self.where_filter.push(filter);
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::query::{NestFilterWrapper, QueryWrapper};

    #[test]
    fn test_simple_query_sql() {
        let target_sql = "SELECT NAME,AGE AS userAge,2 as userStatus FROM BMBP_USERS";
        let mut query_wrapper = QueryWrapper::new();
        query_wrapper.select("NAME").select_alias("AGE", "userAge").select_alias("2", "userStatus");
        query_wrapper.from("BMBP_USERS");
        assert_eq!(target_sql, query_wrapper.query_sql())
    }

    #[test]
    fn test_nest_filter_simple_and() {
        let target_sql = "( name = 1 and name = 3 and c= 3)";
        let mut nest_filter_wrapper = NestFilterWrapper::new();
        nest_filter_wrapper.eq("name", "1").eq("name", "3").eq("c", "3");
        assert_eq!(target_sql, nest_filter_wrapper.filter_sql())
    }
    #[test]
    fn test_nest_filter_simple_or() {
        let target_sql = "( name = 1 or name = 3 or c= 3 or c=5)";
        let mut nest_filter_wrapper = NestFilterWrapper::new();
        nest_filter_wrapper.eq("name", "1").eq("name", "3").eq("c", "3").eq("c", "5");
        assert_eq!(target_sql, nest_filter_wrapper.filter_sql())
    }
    #[test]
    fn test_nest_filter_simple_or_and() {
        let target_sql = "( name = 1 and name = 3 and c= 3 or c=5 and d=6)";
        let mut nest_filter_wrapper = NestFilterWrapper::new().or();
        nest_filter_wrapper.eq("name", "1").eq("name", "3").eq("c", "3").eq("c", "5").and_eq("d", "6");
        assert_eq!(target_sql, nest_filter_wrapper.filter_sql())
    }


}