use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};

/// 权限名称
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacPrivilege {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 权限编码
    privilege_code: Option<String>,
    /// 权限名称
    privilege_title: Option<String>,
    /// 所属应用
    app_code: Option<String>,
}
