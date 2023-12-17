use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacRoleUser {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,

    /// 角色编码
    role_id: Option<String>,

    /// 用户编码
    user_id: Option<String>,
}

#[allow(dead_code)]
impl BmbpRbacRoleUser {
    pub fn new() -> Self {
        BmbpRbacRoleUser::default()
    }
    pub fn orm_table_name() -> String {
        "bmbp_rbac_role_user".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_table_columns();
        let rbac_app_field = vec!["role_id".to_string(), "user_id".to_string()];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}
