use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;
/// URL 信息
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacUrl {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 应用编码
    app_code: Option<String>,
    /// 菜单编码
    menu_code: Option<String>,
    /// URL编码
    url_code: Option<String>,
    /// URL路径
    url_path: Option<String>,
    /// URL名称
    url_title: Option<String>,
    /// 是否外部服务
    is_api_service: Option<bool>,
}
