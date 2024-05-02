use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacAppRole {
    // 应用主键
    app_id: Option<String>,
    // 应用编码
    app_code: Option<String>,
    // 角色主键
    role_id: Option<String>,
    // 角色编码
    role_code: Option<String>,
}
