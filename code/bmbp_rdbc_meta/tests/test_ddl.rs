use bmbp_rdbc_meta::{build_create_table_sql, DataBaseType, RdbcTableVo};

#[test]
fn test_create_table_sql() {
    let table = build_table_vo();
    let build_rs = build_create_table_sql(&DataBaseType::POSTGRESQL, &table);
    if build_rs.is_ok() {
        let build_sql = build_rs.unwrap();
        let table_sql = get_table_sql();
        assert_eq!(build_sql, table_sql)
    } else {
        println!("{:?}", build_rs.err().unwrap());
        assert_eq!(false, true)
    }
}

fn build_table_vo() -> RdbcTableVo {
    RdbcTableVo::default()
}
fn get_table_sql() -> String {
    r#"create table "#.to_string()
}
