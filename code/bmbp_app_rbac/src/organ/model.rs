use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};
use std::string::ToString;
/// 组织机构
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrgan {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 组织编码
    organ_code: Option<String>,
    /// 上级组织编码
    organ_parent_code: Option<String>,
    /// 组织名称
    organ_title: Option<String>,
    /// 组织编码路径
    organ_code_path: Option<String>,
    /// 组织名称路径
    organ_title_path: Option<String>,
    /// 惟一数据标识
    organ_data_id: Option<String>,
    /// 组织类型
    organ_type: Option<String>,
    /// 下级组织
    organ_children: Option<Vec<BmbpRbacOrgan>>,
}
#[allow(dead_code)]
impl BmbpRbacOrgan {
    pub fn new() -> Self {
        BmbpRbacOrgan::default()
    }
    pub fn orm_table_name() -> String {
        "bmbp_rbac_organ".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_table_columns();
        let rbac_app_field = vec![
            "organ_code".to_string(),
            "organ_parent_code".to_string(),
            "organ_title".to_string(),
            "organ_code_path".to_string(),
            "organ_title_path".to_string(),
            "organ_data_id".to_string(),
            "organ_type".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}
