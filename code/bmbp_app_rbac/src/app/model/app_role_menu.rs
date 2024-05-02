use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacAppRoleMenu {
    // 应用ID
    app_id: Option<String>,
    // 菜单ID
    menu_id: Option<String>,
    // 角色ID
    role_id: Option<String>,
}
