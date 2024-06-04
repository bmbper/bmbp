use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;
use chrono::Utc;
use salvo::*;
use tracing::*;
#[rdbc_model(BMBP_RBAC_USER)]
pub struct BmbpRbacUser {
    // 组织ID
    #[query(eq)]
    org_id: Option<String>,
    // 组织ID
    #[query(like)]
    org_id_path: Option<String>,
    // 人员ID
    #[query(eq)]
    person_id: Option<String>,
    // 名称
    #[query(like)]
    user_name: Option<String>,
    // 昵称
    #[query(like)]
    nick_name: Option<String>,
    // 手机
    mobile: Option<String>,
    // 邮箱
    email: Option<String>,

    // 密码
    passwd: Option<String>,
    #[rdbc_skip]
    extends: Option<BmbpRbacUserExtend>,
}

#[rdbc_model(BMBP_RBAC_USER)]
pub struct BmbpRbacUserExtend {
    // 用户ID
    user_id: Option<String>,
    // 密码状态
    pwd_status: Option<String>,
    // 锁定状态
    lock_status: Option<String>,
    // 上次修改密码时间
    pwd_modify_time: Option<String>,
    // 密码修改次数
    pwd_modify_count: Option<u32>,
    // 密码过期时间
    pwd_expire_time: Option<String>,
}
