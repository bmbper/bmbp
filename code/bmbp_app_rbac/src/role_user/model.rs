use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacRoleUser {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,

    /// 角色编码
    role_code: Option<String>,

    /// 用户编码
    user_code: Option<String>,
}

#[allow(dead_code)]
impl RbacRoleUser {
    pub fn new() -> Self {
        RbacRoleUser::default()
    }
    pub fn set_base_model(&mut self, base_model: BmbpBaseModel) -> &mut Self {
        self.base = base_model;
        self
    }
    pub fn get_mut_base_model(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }
    pub fn get_base_model(&self) -> &BmbpBaseModel {
        &self.base
    }

    pub fn set_role_code(&mut self, role_code: String) -> &mut Self {
        self.role_code = Some(role_code);
        self
    }
    pub fn get_role_code(&self) -> Option<&String> {
        self.role_code.as_ref()
    }

    pub fn set_user_code(&mut self, user_code: String) -> &mut Self {
        self.user_code = Some(user_code);
        self
    }
    pub fn get_user_code(&self) -> Option<&String> {
        self.user_code.as_ref()
    }
}
