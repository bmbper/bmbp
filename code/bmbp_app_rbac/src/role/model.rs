use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacRole {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 角色编码
    role_code: Option<String>,
    /// 角色上级编码
    role_parent_code: Option<String>,
    /// 角色名称
    role_title: Option<String>,
    /// 角色编码路径
    role_code_path: Option<String>,
    /// 角色名称路径
    role_title_path: Option<String>,
    /// 角色类型
    role_type: BmbpRoleType,
    /// 下级角色
    role_children: Option<Vec<BmbpRbacRole>>,
    // 授权数量
    role_user_count: Option<usize>,
    // 解色关联类型，授权时是否授予关联角色
    role_ref_type: BmbpRoleRefType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRoleType {
    ADMIN,
    NORMAL,
}
impl Default for BmbpRoleType {
    fn default() -> Self {
        BmbpRoleType::NORMAL
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRoleRefType {
    PARENT,
    CHILD,
    ALL,
    ONLY,
}
impl Default for BmbpRoleRefType {
    fn default() -> Self {
        BmbpRoleRefType::ALL
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacRoleExtend {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 角色编码
    role_code: Option<String>,
    /// 互斥角色编码
    exclusive_role_code: Option<String>,
}
