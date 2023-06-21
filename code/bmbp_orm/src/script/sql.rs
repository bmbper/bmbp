/// SQL脚本构建器，Mybatis的动态SQL构建器
/// 复杂语法需要自己手动编写片断之后 组装到SQL当中
/// 构建器仅负责SQL组装，具体解析法由SqlParser解析器负责
/// # example
/// ```
///
/// let mut sql = BmbpSimpleSQL::new();///
/// let sql_string = sql.select("name").select("xxx").from("xx").filter("ddd=xxx");
///
/// ```
pub enum BmbpScriptType {
    INSERT,
    UPDATE,
    DELETE,
    QUERY,
}

#[allow(unused)]
pub struct BmbpScriptSql {
    script_type: BmbpScriptType,
    from_tables: Vec<String>,
    join_tables: Vec<String>,
    insert_tables: Vec<String>,
    update_tables: Vec<String>,
    delete_tables: Vec<String>,
    update_set: Vec<String>,
    select_fields: Vec<String>,
    and_filters: Vec<String>,
    or_filters: Vec<String>,
    order_fields: Vec<String>,
    group_files: Vec<String>,
    insert_columns: Vec<String>,
    insert_values: Vec<String>,
    set_values: Vec<String>,
}

impl BmbpScriptSql {
    pub fn new() -> Self {
        BmbpScriptSql {
            script_type: BmbpScriptType::QUERY,
            from_tables: vec![],
            insert_tables: vec![],
            update_tables: vec![],
            delete_tables: vec![],
            update_set: vec![],
            select_fields: vec![],
            and_filters: vec![],
            or_filters: vec![],
            group_files: vec![],
            insert_columns: vec![],
            insert_values: vec![],
            order_fields: vec![],
            set_values: vec![],
            join_tables: vec![],
        }
    }
    /// 输出SQL脚本
    pub fn to_script(&self) -> String {
        self.to_sql_string()
    }
    pub fn to_sql_string(&self) -> String {
        match self.script_type {
            BmbpScriptType::INSERT => self.to_insert_sql(),
            BmbpScriptType::UPDATE => self.to_update_sql(),
            BmbpScriptType::DELETE => self.to_delete_sql(),
            BmbpScriptType::QUERY => self.to_query_sql(),
        }
    }
    pub fn to_insert_sql(&self) -> String {
        let mut insert_vec = vec![];
        if !self.insert_tables.is_empty() {
            insert_vec.push("INSERT INTO".to_string());
            insert_vec.push(self.insert_tables.join(","))
        }
        if !self.insert_columns.is_empty() {
            insert_vec.push("(".to_string());
            insert_vec.push(self.insert_columns.join(","));
            insert_vec.push(")".to_string());
        }
        if !self.insert_values.is_empty() {
            insert_vec.push("VALUES(".to_string());
            insert_vec.push(self.insert_values.join(","));
            insert_vec.push(")".to_string());
        }
        insert_vec.join(" ")
    }

    pub fn to_update_sql(&self) -> String {
        let mut update_vec = vec![];
        if !self.update_tables.is_empty() {
            update_vec.push("UPDATE ".to_string());
            update_vec.push(self.update_tables.join(","))
        }
        if !self.set_values.is_empty() {
            update_vec.push("SET".to_string());
            update_vec.push(self.set_values.join(","))
        }
        let mut filter_vec = vec![];
        for field in self.and_filters.as_slice() {
            let mut and_filter_vec = vec![];
            and_filter_vec.push(format!("({})", field));
            filter_vec.push(and_filter_vec.join(" AND "));
        }
        for field in self.or_filters.as_slice() {
            let mut or_filter_vec = vec![];
            or_filter_vec.push(format!("({})", field));
            filter_vec.push(or_filter_vec.join(" OR "));
        }

        if !filter_vec.is_empty() {
            update_vec.push("WHERE".to_string());
            update_vec.push(filter_vec.join(" AND "));
        }
        update_vec.join(" ")
    }

    pub fn to_delete_sql(&self) -> String {
        let mut delete_vec = vec![];
        if !self.delete_tables.is_empty() {
            delete_vec.push("DELETE FROM".to_string());
            delete_vec.push(self.delete_tables.join(","))
        }

        let mut filter_vec = vec![];
        for field in self.and_filters.as_slice() {
            let mut and_filter_vec = vec![];
            and_filter_vec.push(format!("({})", field));
            filter_vec.push(and_filter_vec.join(" AND "));
        }
        for field in self.or_filters.as_slice() {
            let mut or_filter_vec = vec![];
            or_filter_vec.push(format!("({})", field));
            filter_vec.push(or_filter_vec.join(" OR "));
        }

        if !filter_vec.is_empty() {
            delete_vec.push("WHERE".to_string());
            delete_vec.push(filter_vec.join(" AND "));
        }

        delete_vec.join(" ")
    }

    pub fn to_query_sql(&self) -> String {
        let mut query_vec = vec![];
        if !self.select_fields.is_empty() {}
        {
            query_vec.push("SELECT".to_string());
            query_vec.push(self.select_fields.join(","));
        }
        if !self.from_tables.is_empty() {
            query_vec.push("FROM".to_string());
            query_vec.push(self.from_tables.join(","));
        }
        if !self.join_tables.is_empty() {
            query_vec.push(self.join_tables.join("    "));
        }
        let mut filter_vec = vec![];
        for field in self.and_filters.as_slice() {
            let mut and_filter_vec = vec![];
            and_filter_vec.push(format!("({})", field));
            filter_vec.push(and_filter_vec.join(" AND "));
        }
        for field in self.or_filters.as_slice() {
            let mut or_filter_vec = vec![];
            or_filter_vec.push(format!("({})", field));
            filter_vec.push(or_filter_vec.join(" OR "));
        }

        if !filter_vec.is_empty() {
            query_vec.push("WHERE".to_string());
            query_vec.push(filter_vec.join(" AND "));
        }

        if !self.group_files.is_empty() {
            query_vec.push("GROUP BY".to_string());
            query_vec.push(self.group_files.join(","));
        }

        if !self.order_fields.is_empty() {
            query_vec.push("ORDER BY".to_string());
            query_vec.push(self.order_fields.join(","));
        }
        query_vec.join(" ")
    }
}

impl BmbpScriptSql {
    pub fn select(&mut self, field: &str) -> &mut Self {
        self.select_fields.push(field.to_string());
        self
    }
    pub fn select_slice(&mut self, field_slice: &[&str]) -> &mut Self {
        for field in field_slice {
            self.select_fields.push(field.to_string());
        }
        self
    }
    pub fn select_string_slice(&mut self, field_slice: &[String]) -> &mut Self {
        for field in field_slice {
            self.select_fields.push(field.to_string());
        }
        self
    }
    pub fn from(&mut self, table: &str) -> &mut Self {
        self.script_type = BmbpScriptType::QUERY;
        self.from_tables.push(table.to_string());
        self
    }
    pub fn from_slice(&mut self, tables: &[&str]) -> &mut Self {
        self.script_type = BmbpScriptType::QUERY;
        for table in tables {
            self.from_tables.push(table.to_string());
        }
        self
    }

    pub fn delete(&mut self, table: &str) -> &mut Self {
        self.script_type = BmbpScriptType::DELETE;
        self.delete_tables.push(table.to_string());
        self
    }
    pub fn delete_slice(&mut self, tables: &[&str]) -> &mut Self {
        self.script_type = BmbpScriptType::DELETE;
        for table in tables {
            self.delete_tables.push(table.to_string())
        }
        self
    }

    pub fn update(&mut self, table: &str) -> &mut Self {
        self.script_type = BmbpScriptType::UPDATE;
        self.update_tables.push(table.to_string());
        self
    }
    pub fn update_slice(&mut self, tables: &[&str]) -> &mut Self {
        self.script_type = BmbpScriptType::UPDATE;
        for table in tables {
            self.update_tables.push(table.to_string());
        }
        self
    }
    pub fn insert_into(&mut self, table: &str) -> &mut Self {
        self.script_type = BmbpScriptType::INSERT;
        self.insert_tables.push(table.to_string());
        self
    }
    pub fn insert_into_slice(&mut self, tables: &[&str]) -> &mut Self {
        self.script_type = BmbpScriptType::INSERT;
        for table in tables {
            self.insert_tables.push(table.to_string());
        }
        self
    }
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.and_filters.push(filter.to_string());
        self
    }

    pub fn filter_slice(&mut self, filter: &[&str]) -> &mut Self {
        for item in filter {
            self.and_filters.push(item.to_string());
        }
        self
    }

    pub fn filter_and(&mut self, filter: &str) -> &mut Self {
        self.and_filters.push(filter.to_string());
        self
    }

    pub fn filter_and_slice(&mut self, filter: &[&str]) -> &mut Self {
        for item in filter {
            self.and_filters.push(item.to_string());
        }
        self
    }

    pub fn filter_or(&mut self, filter: &str) -> &mut Self {
        self.or_filters.push(filter.to_string());
        self
    }

    pub fn filter_or_slice(&mut self, filter: &[&str]) -> &mut Self {
        for item in filter {
            self.or_filters.push(item.to_string());
        }
        self
    }

    pub fn join(&mut self, join_table: &str) -> &mut Self {
        self.from_tables.push(join_table.to_string());
        self
    }

    pub fn left_join(&mut self, join_table: &str) -> &mut Self {
        self.join_tables.push(format!("LEFT JOIN {}", join_table));
        self
    }

    pub fn right_join(&mut self, join_table: &str) -> &Self {
        self.join_tables.push(format!("RIGHT JOIN {}", join_table));
        self
    }

    pub fn order_by(&mut self, order: &str) -> &mut Self {
        self.order_fields.push(order.to_string());
        self
    }

    pub fn order_by_slice(&mut self, order: &str) -> &mut Self {
        self.join_tables.push(order.to_string());
        self
    }

    pub fn group_by(&mut self, field: &str) -> &mut Self {
        self.group_files.push(field.to_string());
        self
    }

    pub fn group_by_slice(&mut self, fields: &[&str]) -> &mut Self {
        for item in fields {
            self.group_files.push(item.to_string());
        }
        self
    }
    pub fn set(&mut self, field: &str) -> &mut Self {
        self.set_values.push(field.to_string());
        self
    }
    pub fn set_value(&mut self, field: &str, value: &str) -> &mut Self {
        self.set_values.push(format!("{}={}", field, value));
        self
    }
    pub fn set_slice(&mut self, fields: &[&str]) -> &mut Self {
        for item in fields {
            self.set_values.push(item.to_string());
        }
        self
    }
    pub fn column(&mut self, column: &str) -> &mut Self {
        self.insert_columns.push(column.to_string());
        self
    }
    pub fn columns(&mut self, columns: &[&str]) -> &mut Self {
        for item in columns {
            self.insert_columns.push(item.to_string());
        }
        self
    }
    pub fn value(&mut self, field: &str) -> &Self {
        self.insert_values.push(field.to_string());
        self
    }
    pub fn values(&mut self, fields: &[&str]) -> &mut Self {
        for item in fields {
            self.insert_values.push(item.to_string());
        }
        self
    }

    pub fn insert_value(&mut self, column: &str, value: &str) -> &mut Self {
        self.insert_columns.push(column.to_string());
        self.insert_values.push(value.to_string());
        self
    }
    pub fn insert_values(&mut self, insert_values: &[(&str, &str)]) -> &mut Self {
        for (column, value) in insert_values {
            self.insert_columns.push(column.to_string());
            self.insert_values.push(value.to_string());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::script::BmbpScriptSql;

    #[test]
    fn test_select_sql() {
        let sql_str = "SELECT * FROM BMBP_RBAC_ORGAN WHERE (ORGAN_ID = #{organId})";
        let mut simple_sql = &mut BmbpScriptSql::new();
        simple_sql = simple_sql
            .select("*")
            .from("BMBP_RBAC_ORGAN")
            .filter("ORGAN_ID = #{organId}");
        let simple_sql_string = &simple_sql.to_sql_string();
        assert_eq!(sql_str, simple_sql_string.as_str())
    }
}
