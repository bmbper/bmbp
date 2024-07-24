use bmbp_marco_rdbc::{table, table_orm};

#[test]
pub fn test_table() {
    use serde::Deserialize;
    use serde::Serialize;
    #[table(bmbp_test)]
    pub struct TestBean {
        #[id]
        id: Option<String>,
        #[column(name = "user_name")]
        name: Option<String>,
    }
    let bean = TestBean::new();
    assert_eq!(TestBean::get_table_name(), "bmbp_test");
}
