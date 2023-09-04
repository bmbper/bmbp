use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacRolePrivilege {
    /// 公共
    #[serde(flatten)]
    base: BmbpBaseModel,

    /// 角色编码
    role_code: Option<String>,

    ///  权限编码
    privilege_code: Option<String>,
}

#[allow(dead_code)]
impl RbacRolePrivilege {
    pub fn new() -> Self {
        RbacRolePrivilege::default()
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

    pub fn set_privilege_code(&mut self, privilege_code: String) -> &mut Self {
        self.privilege_code = Some(privilege_code);
        self
    }
    pub fn get_privilege_code(&self) -> Option<&String> {
        self.privilege_code.as_ref()
    }
}
