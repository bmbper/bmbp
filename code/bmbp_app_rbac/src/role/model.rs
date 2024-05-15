use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;
use bmbp_rdbc_orm::{BmbpRdbcModel, RdbcModel, RdbcOrmRow};
use chrono::Utc;
use salvo::*;
use tracing::*;

#[rdbc_model(table = BMBP_RBAC_ROLE)]
pub struct BmbpRbacRole {
    role_code: Option<String>,
    role_name: Option<String>,
    role_desc: Option<String>,
}
