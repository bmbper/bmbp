use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};

/// 权限名称
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacPrivilegeUrl {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 权限编码
    privilege_code: Option<String>,
    /// URL编码
    url_code: Option<String>,
    /// 校验权重
    url_wight: Option<usize>,
    /// 校验类型
    url_valid: BmbpUrlValidType,
}

/// URL 校验类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpUrlValidType {
    /// 通过
    ACEESS,
    /// 拒绝
    REJECT,
}

impl Default for BmbpUrlValidType {
    fn default() -> Self {
        BmbpUrlValidType::ACEESS
    }
}
