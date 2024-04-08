use serde::{Deserialize, Serialize};

use bmbp_rdbc_orm::{RdbcModel, RdbcOrmRow};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacUser {
    // 组织ID
    org_id: Option<String>,
    // 人员ID
    person_id: Option<String>,
    // 名称
    user_name: Option<String>,
    // 手机
    mobile: Option<String>,
    // 邮箱
    email: Option<String>,
    // 昵称
    nick_name: Option<String>,
    // 密码
    password: Option<String>,

    extends: Option<BmbpRbacUserExtend>,
}

impl RdbcModel for BmbpRbacUser {
    fn get_table_name() -> String {
        "bmbp_rbac_user".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}

impl From<RdbcOrmRow> for BmbpRbacUser {
    fn from(row: RdbcOrmRow) -> Self {
        BmbpRbacUser {
            org_id: None,
            person_id: None,
            user_name: None,
            mobile: None,
            email: None,
            nick_name: None,
            password: None,
            extends: None,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacUserExtend {
    // 用户ID
    user_id: Option<String>,
    // 密码状态
    password_status: Option<String>,
    // 锁定状态
    lock_status: Option<String>,
    // 上次修改密码时间
    last_modify_password_time: Option<String>,
    // 密码修改次数
    password_modify_count: Option<u16>,
    // 密码过期时间
    password_expire_time: Option<String>,
}

impl RdbcModel for BmbpRbacUserExtend {
    fn get_table_name() -> String {
        "bmbp_rbac_user_extend".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}

impl From<RdbcOrmRow> for BmbpRbacUserExtend {
    fn from(value: RdbcOrmRow) -> Self {
        BmbpRbacUserExtend {
            user_id: None,
            password_status: None,
            lock_status: None,
            last_modify_password_time: None,
            password_modify_count: None,
            password_expire_time: None,
        }
    }
}
