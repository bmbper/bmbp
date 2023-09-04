use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};
use std::string::ToString;
/// 菜单机构
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacMenu {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 应用ID
    app_id: Option<String>,
    /// 菜单编码
    menu_code: Option<String>,
    /// 上级菜单编码
    menu_parent_code: Option<String>,
    /// 菜单名称
    menu_title: Option<String>,
    /// 菜单编码路径
    menu_code_path: Option<String>,
    /// 菜单名称路径
    menu_title_path: Option<String>,
    /// 惟一数据标识
    menu_url: Option<String>,
    /// 菜单类型
    menu_type: Option<String>,
    /// 下级菜单
    menu_children: Option<Vec<BmbpRbacMenu>>,
}
#[allow(dead_code)]
impl BmbpRbacMenu {
    pub fn new() -> Self {
        BmbpRbacMenu::default()
    }
    pub fn orm_table_name() -> String {
        "bmbp_rbac_menu".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_fields();
        let rbac_app_field = vec![
            "app_id".to_string(),
            "menu_code".to_string(),
            "menu_parent_code".to_string(),
            "menu_title".to_string(),
            "menu_code_path".to_string(),
            "menu_title_path".to_string(),
            "menu_url".to_string(),
            "menu_type".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}
