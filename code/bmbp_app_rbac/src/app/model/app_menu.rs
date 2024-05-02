use serde::{Deserialize, Serialize};

use crate::app::model::types::{RbacAppMenuOpenType, RbacAppMenuType};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacAppMenu {
    // 应用ID
    app_id: Option<String>,
    // 应用编码
    app_code: Option<String>,
    // 菜单类型
    menu_type: Option<RbacAppMenuType>,
    // 打开类型
    open_type: Option<RbacAppMenuOpenType>,
    // 菜单图标
    menu_icon: Option<String>,
    // 菜单地址
    menu_url: Option<String>,
}
