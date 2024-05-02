use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RbacAppType {
    // 内置应用、配置应用
    META,
    // 拖管应用
    MANAGED,
    // 单点集成应用
    SSO,
    // 外部应用链接
    LINK,
}
impl Default for RbacAppType {
    fn default() -> Self {
        RbacAppType::META
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RbacAppMenuType {
    // 配置页面
    META,
    // 路由页面
    ROUTE,
    // 链接页面
    LINK,
}
impl Default for RbacAppMenuType {
    fn default() -> Self {
        RbacAppMenuType::META
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RbacAppMenuOpenType {
    // 应用内部打开
    ROUTE,
    // 弹出窗口打开
    WINDOW,
    // 外部选项卡打开
    LINK,
}
impl Default for RbacAppMenuOpenType {
    fn default() -> Self {
        RbacAppMenuOpenType::ROUTE
    }
}
