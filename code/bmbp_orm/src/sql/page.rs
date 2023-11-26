use std::fmt::format;

fn to_general_count_sql(sql: &str) -> String {
    return format!("SELECT COUNT(1) AS COUNT FROM ( {} ) AS TEMP ", sql);
}

pub fn to_mysql_count_sql(sql: &str) -> String {
    return to_general_count_sql(sql);
}

pub fn to_pg_count_sql(sql: &str) -> String {
    return to_general_count_sql(sql);
}

pub fn to_oracle_sql(sql:&str)->String{
    return format!("SELECT COUNT(1) AS COUNT FROM ( {} ) AS TEMP ",sql)
}


#[cfg(test)]
mod test {
    use crate::sql::page::to_general_count_sql;

    #[test]
    fn test_general_count_sql() {
        let sql = "SELECT 1";
        let general_count_sql = to_general_count_sql(sql);
        assert_eq!(
            "SELECT COUNT(1) AS COUNT FROM ( SELECT 1 ) AS TEMP ",
            general_count_sql.as_str()
        )
    }
}
