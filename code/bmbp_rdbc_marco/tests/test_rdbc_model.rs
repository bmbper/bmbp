use bmbp_rdbc_marco::rdbc_model;

#[test]
fn test_rdbc_model_table() {
    use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcFilter, RdbcTable, Update};
    use serde::{Deserialize, Serialize};
    #[rdbc_model(table = RDBC_MODEL_TEST)]
    pub struct RdbcModel {
        pub name: String,
        pub age: Option<i32>,
        #[rdbc_skip]
        pub data_id: Option<u32>,
        #[rdbc_skip]
        children: Vec<RdbcModel>,
    }
}
#[test]
fn test_rdbc_model_empty() {
    use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcFilter, RdbcTable, Update};
    use serde::{Deserialize, Serialize};
    #[rdbc_model]
    pub struct RdbcModel {
        name: String,
        age: Option<i32>,
    }
}
