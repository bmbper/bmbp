use bmbp_rdbc_orm::RdbcValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<&RbacAppType> for RdbcValue {
    fn from(value: &RbacAppType) -> Self {
        match value {
            RbacAppType::META => RdbcValue::String("META".to_string()),
            RbacAppType::MANAGED => RdbcValue::String("MANAGED".to_string()),
            RbacAppType::SSO => RdbcValue::String("SSO".to_string()),
            RbacAppType::LINK => RdbcValue::String("LINK".to_string()),
        }
    }
}

impl Into<RbacAppType> for &RdbcValue {
    fn into(self) -> RbacAppType {
        let meta = self.get_string();
        if meta == "META" {
            return RbacAppType::META;
        } else if meta == "MANAGED" {
            return RbacAppType::MANAGED;
        } else if meta == "SSO" {
            return RbacAppType::SSO;
        } else if meta == "LINK" {
            return RbacAppType::LINK;
        }
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
