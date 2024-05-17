use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::table;

#[test]
fn test_rdbc_model_empty() {
    use ::serde::Deserialize;
    use ::serde::Serialize;
    use bmbp_app_common::BmbpError;
    use bmbp_app_common::BmbpPageParam;
    use bmbp_app_common::BmbpResp;
    use bmbp_app_common::HttpRespListVo;
    use bmbp_app_common::HttpRespPageVo;
    use bmbp_app_common::HttpRespVo;
    use bmbp_app_common::PageVo;
    use bmbp_app_common::RespVo;
    use bmbp_rdbc_orm::RdbcMacroTree;
    use bmbp_rdbc_orm::RdbcORM;
    use bmbp_rdbc_orm::RdbcOrmRow;
    use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcFilter, RdbcTable, Update};
    use chrono::Utc;
    use salvo::*;
    #[rdbc_model(table = BMBP_RBAC_ROLE,tree=role)]
    pub struct BmbpRbacRole {
        role_code: Option<String>,
        #[query(like)]
        role_name: Option<String>,
        role_desc: Option<String>,
    }
}
