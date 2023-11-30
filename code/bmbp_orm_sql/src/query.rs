/// 查询构建器
///
#[derive(Clone,Default)]
pub struct QueryWrapper<'a>{
    selected_field:Vec<&'a str>,
    from_table:Vec<&'a str>,
}
impl QueryWrapper<'_> {
    pub fn new() -> QueryWrapper<'static> {
        QueryWrapper{
            selected_field:vec![],
            from_table:vec![],
        }
    }
    pub fn query_sql(&self) -> String {
      "".to_string()
    }
}

impl QueryWrapper<'_> {
    pub fn select(&mut self, field:&str) -> &mut Self {
        self.selected_field.push(field);
        self
    }
    pub fn select_alias(&mut self, field:&str,alias:&str) -> &mut Self {
        self.selected_field.push(format!("{} as {}",field,alias).as_str());
        self
    }
    pub fn from(&mut self, table:&str) -> &mut Self {
        self.from_table.push(table);
        self
    }
}




#[cfg(test)]
mod tests {
    use crate::query::QueryWrapper;

    #[test]
    fn test_simple_query_sql() {
        let target_sql="SELECT NAME,AGE AS userAge,2 as userStatus FROM BMBP_USERS";
        let mut query_wrapper = QueryWrapper::new();
        query_wrapper.select("NAME").select_alias("AGE","userAge").select_alias("2","userStatus");
        query_wrapper.from("BMBP_USERS");
        assert_eq!(target_sql,query_wrapper.query_sql())
    }
}