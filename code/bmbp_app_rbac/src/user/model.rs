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
impl BmbpRbacUser {
    pub fn new() -> Self {
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
    pub fn get_org_id(&self) -> Option<&String> {
        self.org_id.as_ref()
    }
    pub fn set_org_id(&mut self, org_id: String) -> &mut Self {
        self.org_id = Some(org_id);
        self
    }
    pub fn get_person_id(&self) -> Option<&String> {
        self.person_id.as_ref()
    }
    pub fn set_person_id(&mut self, person_id: String) -> &mut Self {
        self.person_id = Some(person_id);
        self
    }
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
    pub fn set_user_name(&mut self, user_name: String) -> &mut Self {
        self.user_name = Some(user_name);
        self
    }
    pub fn get_mobile(&self) -> Option<&String> {
        self.mobile.as_ref()
    }
    pub fn set_mobile(&mut self, mobile: String) -> &mut Self {
        self.mobile = Some(mobile);
        self
    }
    pub fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn get_nick_name(&self) -> Option<&String> {
        self.nick_name.as_ref()
    }
    fn set_nick_name(&mut self, nick_name: String) -> &mut Self {
        self.nick_name = Some(nick_name);
        self
    }
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }
    fn set_password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }
    pub fn get_extends(&self) -> Option<&BmbpRbacUserExtend> {
        self.extends.as_ref()
    }
    pub fn get_extends_mut(&mut self) -> Option<&mut BmbpRbacUserExtend> {
        self.extends.as_mut()
    }
    fn set_extends(&mut self, extends: BmbpRbacUserExtend) -> &mut Self {
        self.extends = Some(extends);
        self
    }
}
impl RdbcModel for BmbpRbacUser {
    fn get_table_name() -> String {
        "bmbp_rbac_user".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "org_id".to_string(),
            "person_id".to_string(),
            "user_name".to_string(),
            "mobile".to_string(),
            "email".to_string(),
            "nick_name".to_string(),
            "password".to_string(),
        ]
    }
}
impl From<RdbcOrmRow> for BmbpRbacUser {
    fn from(row: RdbcOrmRow) -> Self {
        let user = BmbpRbacUser {
            org_id: None,
            person_id: None,
            user_name: None,
            mobile: None,
            email: None,
            nick_name: None,
            password: None,
            extends: None,
        };
        user
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

impl BmbpRbacUserExtend {
    pub fn new() -> Self {
        BmbpRbacUserExtend {
            user_id: None,
            password_status: None,
            lock_status: None,
            last_modify_password_time: None,
            password_modify_count: None,
            password_expire_time: None,
        }
    }
    pub fn get_user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }
    fn set_user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn get_password_status(&self) -> Option<&String> {
        self.password_status.as_ref()
    }
    fn set_password_status(&mut self, password_status: String) -> &mut Self {
        self.password_status = Some(password_status);
        self
    }
    pub fn get_lock_status(&self) -> Option<&String> {
        self.lock_status.as_ref()
    }
    fn set_lock_status(&mut self, lock_status: String) -> &mut Self {
        self.lock_status = Some(lock_status);
        self
    }
    pub fn get_last_modify_password_time(&self) -> Option<&String> {
        self.last_modify_password_time.as_ref()
    }
    fn set_last_modify_password_time(&mut self, last_modify_password_time: String) -> &mut Self {
        self.last_modify_password_time = Some(last_modify_password_time);
        self
    }
    pub fn get_password_modify_count(&self) -> Option<&u16> {
        self.password_modify_count.as_ref()
    }
    fn set_password_modify_count(&mut self, password_modify_count: u16) -> &mut Self {
        self.password_modify_count = Some(password_modify_count);
        self
    }
    pub fn get_password_expire_time(&self) -> Option<&String> {
        self.password_expire_time.as_ref()
    }
    fn set_password_expire_time(&mut self, password_expire_time: String) -> &mut Self {
        self.password_expire_time = Some(password_expire_time);
        self
    }
}

impl RdbcModel for BmbpRbacUserExtend {
    fn get_table_name() -> String {
        "bmbp_rbac_user_extend".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "user_id".to_string(),
            "password_status".to_string(),
            "lock_status".to_string(),
            "last_modify_password_time".to_string(),
            "password_modify_count".to_string(),
            "password_expire_time".to_string(),
        ]
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
