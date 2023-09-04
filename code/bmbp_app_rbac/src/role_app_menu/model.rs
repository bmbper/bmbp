use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};

/// 权限名称
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacPrivilegeUI {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 权限编码
    privilege_code: Option<String>,
    /// 界面编码
    ui_code: Option<String>,
    /// 界面类型
    ui_type: BmbpRbacUiType,
    /// 是否可见
    is_visible: Option<bool>,
    /// 是否只读
    is_read_only: Option<bool>,
}

/// URL 校验类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRbacUiType {
    /// 模块菜单
    MENU,
    /// 页面
    PAGE,
    /// 按钮
    BUTTON,
    /// 字段集合
    FIELDGROUP,
    /// 字段
    FIELD,
}

impl Default for BmbpRbacUiType {
    fn default() -> Self {
        BmbpRbacUiType::MENU
    }
}
