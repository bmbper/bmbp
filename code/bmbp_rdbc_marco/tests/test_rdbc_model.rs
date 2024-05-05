use bmbp_rdbc_marco::rdbc_model;

#[test]
fn test_rdbc_model_empty() {
    use ::serde::Deserialize;
    use ::serde::Serialize;
    use bmbp_app_common::BmbpError;
    use bmbp_app_common::BmbpResp;
    use bmbp_app_common::HttpRespListVo;
    use bmbp_app_common::HttpRespPageVo;
    use bmbp_app_common::HttpRespVo;
    use bmbp_app_common::PageVo;
    use bmbp_app_common::RespVo;
    use bmbp_rdbc_orm::RdbcORM;
    use bmbp_rdbc_orm::RdbcOrmRow;
    use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcFilter, RdbcTable, Update};
    use salvo::*;
    #[rdbc_model]
    pub struct RdbcModel {
        name: String,
        age: Option<i32>,
    }
}
