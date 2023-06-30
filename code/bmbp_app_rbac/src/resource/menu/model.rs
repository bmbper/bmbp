use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacMenu {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 菜单编码
    menu_code: Option<String>,
    /// 菜单上级编码
    menu_parent_code: Option<String>,
    /// 菜单名称
    menu_title: Option<String>,
    /// 菜单编码路径
    menu_code_path: Option<String>,
    /// 菜单名称路径
    menu_title_path: Option<String>,
    /// 菜单类型
    menu_type: BmbpMenuType,
    /// 菜单角色
    menu_children: Option<Vec<BmbpRbacMenu>>,

    /// 菜单路由类型
    route_type: BmbpRouteType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpMenuType {
    MODULE,
    FUNC,
}
impl Default for BmbpMenuType {
    fn default() -> Self {
        BmbpMenuType::MODULE
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRouteType {
    URL,
    ROUTE,
}
impl Default for BmbpRouteType {
    fn default() -> Self {
        BmbpRouteType::ROUTE
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
