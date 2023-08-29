use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};
use std::string::ToString;
/// 组织机构
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacUser {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 组织编码
    organ_code: Option<String>,
    user_name: Option<String>,
    user_nick_name: Option<String>,
    user_password: Option<String>,
}
#[allow(dead_code)]
impl BmbpRbacUser {
    pub fn new() -> Self {
        BmbpRbacUser::default()
    }
    pub fn orm_table_name() -> String {
        "bmbp_rbac_user".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_fields();
        let rbac_app_field = vec![
            "organ_code".to_string(),
            "user_name".to_string(),
            "user_passowrd".to_string(),
            "user_nick_name".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}
