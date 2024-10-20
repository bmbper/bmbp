use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_app_orm::*;
use bmbp_marco::rdbc_model;
use chrono::Utc;
use salvo::*;
use tracing::*;
#[rdbc_model(table = BMBP_RBAC_ROLE)]
pub struct BmbpRbacRole {
    role_code: Option<String>,
    #[query(like)]
    role_name: Option<String>,
    role_desc: Option<String>,
}
