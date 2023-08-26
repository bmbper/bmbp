use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};
use std::string::ToString;
/// 组织机构
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacRole {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 组织编码
    role_code: Option<String>,
    /// 上级组织编码
    role_parent_code: Option<String>,
    /// 组织名称
    role_title: Option<String>,
    /// 组织编码路径
    role_code_path: Option<String>,
    /// 组织名称路径
    role_title_path: Option<String>,
    /// 惟一数据标识
    role_data_id: Option<String>,
    /// 组织类型
    role_type: Option<String>,
    /// 下级组织
    role_children: Option<Vec<BmbpRbacRole>>,
}
#[allow(dead_code)]
impl BmbpRbacRole {
    pub fn new() -> Self {
        BmbpRbacRole::default()
    }
    pub fn orm_table_name() -> String {
        "bmbp_rbac_role".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_fields();
        let rbac_app_field = vec![
            "role_code".to_string(),
            "role_parent_code".to_string(),
            "role_title".to_string(),
            "role_code_path".to_string(),
            "role_title_path".to_string(),
            "role_data_id".to_string(),
            "role_type".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}
