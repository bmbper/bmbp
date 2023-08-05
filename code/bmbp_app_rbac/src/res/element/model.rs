use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

/// 菜单元素
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacMenuElement {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 元素编码
    element_code: Option<String>,
    /// 元素上级编码
    element_parent_code: Option<String>,
    /// 元素名称
    element_title: Option<String>,
    /// 元素编码路径
    element_code_path: Option<String>,
    /// 元素名称路径
    element_title_path: Option<String>,
    /// 元素类型： 页面、按钮、区域、字段
    element_type: BmbpMenuElemenutType,
    /// 下级元素
    element_children: Option<Vec<BmbpRbacMenuElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpMenuElemenutType {
    PAGE,
    BUTTON,
    FIELDGROUP,
    FIELD,
}

impl Default for BmbpMenuElemenutType {
    fn default() -> Self {
        BmbpMenuElemenutType::PAGE
    }
}
