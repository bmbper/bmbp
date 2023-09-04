use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};

/// 权限名称
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacPrivilegeDataScope {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 权限编码
    privilege_code: Option<String>,
    /// 数据权限编码
    scope_code: Option<String>,
    /// 过虑类型
    scope_type: BmbpRbacDataScopeValidType,
}

/// URL 校验类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRbacDataScopeValidType {
    /// 包含
    INCLUDE,
    /// 排除
    EXCLUDE,
    /// 忽略
    IGNORE,
}

impl Default for BmbpRbacDataScopeValidType {
    fn default() -> Self {
        BmbpRbacDataScopeValidType::EXCLUDE
    }
}
