use serde::{Deserialize, Serialize};

use bmbp_rdbc_orm::{BmbpRdbcModel, RdbcModel, RdbcOrmRow};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacRoleQueryParams {
    role_name: Option<String>,
}
impl RbacRoleQueryParams {
    pub fn new() -> Self {
        RbacRoleQueryParams { role_name: None }
    }
    pub fn get_role_name(&self) -> Option<&String> {
        self.role_name.as_ref()
    }
    pub fn set_role_name(&mut self, role_name: String) -> &mut Self {
        self.role_name = Some(role_name);
        self
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacRole {
    role_code: Option<String>,
    role_name: Option<String>,
    role_desc: Option<String>,
}
impl BmbpRbacRole {
    pub fn new() -> Self {
        BmbpRbacRole {
            role_code: None,
            role_name: None,
            role_desc: None,
        }
    }
    pub fn get_role_code(&self) -> Option<&String> {
        self.role_code.as_ref()
    }
    pub fn set_role_code(&mut self, role_code: String) -> &mut Self {
        self.role_code = Some(role_code);
        self
    }
    pub fn get_role_name(&self) -> Option<&String> {
        self.role_name.as_ref()
    }
    pub fn set_role_name(&mut self, role_name: String) -> &mut Self {
        self.role_name = Some(role_name);
        self
    }
    pub fn get_role_desc(&self) -> Option<&String> {
        self.role_desc.as_ref()
    }
    pub fn set_role_desc(&mut self, role_desc: String) -> &mut Self {
        self.role_desc = Some(role_desc);
        self
    }
}

impl RdbcModel for BmbpRbacRole {
    fn get_table_name() -> String {
        "bmbp_rbac_role".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "role_code".to_string(),
            "role_name".to_string(),
            "role_desc".to_string(),
        ]
    }
}

impl From<RdbcOrmRow> for BmbpRbacRole {
    fn from(row: RdbcOrmRow) -> Self {
        let mut role = BmbpRbacRole::new();
        if let Some(data) = row.get_data().get("role_code") {
            role.set_role_code(data.to_string());
        }
        if let Some(data) = row.get_data().get("role_name") {
            role.set_role_name(data.to_string());
        }
        if let Some(data) = row.get_data().get("role_desc") {
            role.set_role_desc(data.to_string());
        }
        role
    }
}

pub type BmbpRbacRoleModel = BmbpRdbcModel<BmbpRbacRole>;
