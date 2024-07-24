use bmbp_marco_rdbc::{table_orm, table_tree_orm};

#[test]
pub fn test_table_tree_orm() {
    use serde::Deserialize;
    use serde::Serialize;
    #[table_tree_orm(bmbp_test, organ)]
    pub struct TestBean {
        #[id]
        id: Option<String>,
        #[column(name = "user_name")]
        name: Option<String>,
    }
    let bean = TestBean::new();
    assert_eq!(TestBean::get_table_name(), "bmbp_test");
    assert_eq!(bean.get_organ_code().is_none(), true);
}
