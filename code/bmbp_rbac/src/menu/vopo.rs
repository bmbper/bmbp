use crate::menu::vopo::BmbpAppType::PLATFORM;
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, TreeNode};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// 应用信息
#[allow(dead_code)]
pub const BMBP_RBAC_APP: &str = "BMBP_RBAC_APP";

// 应用目录信息
#[allow(dead_code)]
pub const BMBP_RBAC_MENU: &str = "BMBP_RBAC_APP_MENU";

// 应用查询参数
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct AppQueryParam {
    r_id: String,
    app_id: String,
    app_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum BmbpAppType {
    PLATFORM,
    LINK,
    INTEGRATION,
}

impl Default for BmbpAppType {
    fn default() -> Self {
        PLATFORM
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BmbpAppVo {
    app_id: String,
    app_title: String,
    app_title_short: String,
    app_icon: String,
    app_desc: String,
    app_type: BmbpAppType,
    app_url: String,
    app_key: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

// 菜单查询参数
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct MenuQueryParam {
    r_id: String,
    menu_id: String,
    menu_title: String,
    menu_parent_id: String,
    menu_path: String,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum BmbpMenuType {
    Route, // 本地应用路由
    URL,   // 接入外部页面地址
    META,  // 配置界面
}

impl Default for BmbpMenuType {
    fn default() -> Self {
        BmbpMenuType::Route
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BmbpMenuVo {
    app_id: String,
    menu_id: String,
    parent_menu_id: String,
    menu_title: String,
    menu_path: String,

    menu_route: String,
    menu_route_type: String,

    children: Vec<BmbpMenuVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl TreeNode<BmbpMenuVo> for BmbpMenuVo {
    fn node_id(&self) -> &String {
        &self.menu_id
    }

    fn node_parent_id(&self) -> &String {
        &self.parent_menu_id
    }

    fn node_title(&self) -> &String {
        &self.menu_title
    }

    fn node_data_id(&self) -> &String {
        &self.menu_id
    }

    fn node_path(&self) -> &String {
        &self.menu_path
    }

    fn children(&self) -> &[BmbpMenuVo] {
        self.children.as_slice()
    }

    fn set_children(&mut self, children: Vec<BmbpMenuVo>) -> &mut Self {
        self.children = children;
        self
    }
}

impl BaseOrmVoPo for BmbpMenuVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) -> &mut Self {
        self.base = vo;
        self
    }

    fn vo_fields() -> Vec<String> {
        vec!["".to_string()]
    }
}
