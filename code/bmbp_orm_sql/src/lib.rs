/// SQL构建器
mod types;
mod sql;
mod build;
mod builder;

pub use types::*;


#[cfg(test)]
mod test {
    use crate::sql::SQL;
    use crate::types::SqlBuilder;

    #[test]
    pub fn test_query_simple() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_where() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' AND NAME = '3') ";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").eq("ID", "'3'").eq("NAME", "'3'");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3')";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where_order() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3') ORDER BY NAME,AGE";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'")
            .order("NAME").order("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where_order_asc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3') ORDER BY NAME ASC,AGE ASC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'")
            .order_asc("NAME").order_asc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where_order_desc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3') ORDER BY NAME DESC,AGE DESC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'")
            .order_desc("NAME").order_desc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where_group_order_asc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3') GROUP BY AGE,NAME ORDER BY NAME ASC,AGE ASC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'")
            .group("AGE,NAME").order_asc("NAME").order_asc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_simple_or_where_group_order_desc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE (ID = '3' OR NAME = '3') GROUP BY AGE,NAME ORDER BY NAME ASC,AGE DESC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER").or().eq("ID", "'3'").eq("NAME", "'3'")
            .group("AGE").group("NAME").order_asc("NAME").order_desc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_nest_where_group_order_desc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE ((ID = '3' OR NAME = '3') AND AGE = 5) GROUP BY AGE,NAME ORDER BY NAME ASC,AGE DESC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER")
            .or()
            .eq("ID", "'3'")
            .eq("NAME", "'3'")
            .and().eq("AGE", "5")

            .group("AGE").group("NAME").order_asc("NAME").order_desc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }

    #[test]
    pub fn test_query_nest_and_where_group_order_desc() {
        let target_sql = "SELECT NAME,AGE AS userAge FROM RBAC_USER WHERE ((ID = '3' OR NAME = '3') AND AGE = 5 AND (AGE = 5 OR Q = '3' OR W = 5 OR X = '6')) GROUP BY AGE,NAME ORDER BY NAME ASC,AGE DESC";
        let mut query = SQL::query();
        query.select("NAME").select_alias("AGE", "userAge").from("RBAC_USER")
            .or()
            .eq("ID", "'3'")
            .eq("NAME", "'3'")
            .and().eq("AGE", "5");
        let mut nest_filter = query.nest_or();
        nest_filter.eq("AGE", "5").eq("Q", "'3'").eq("W", "5").eq("X", "'6'");
        query.add_filter(nest_filter);
        query.group("AGE").group("NAME").order_asc("NAME").order_desc("AGE");
        let query_sql = query.build().replace("\n", "");
        assert_eq!(target_sql, query_sql.as_str())
    }
}