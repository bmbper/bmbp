use crate::menu::vopo::BmbpAppType::PLATFORM;
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, TreeNode};
use serde::{Deserialize, Serialize};

// 应用信息
#[allow(dead_code)]
pub const BMBP_RBAC_APP: &str = "BMBP_RBAC_APP";

// 应用目录信息
#[allow(dead_code)]
pub const BMBP_RBAC_MENU: &str = "BMBP_RBAC_APP_MENU";

// 应用查询参数
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AppQueryParam {
    r_id: String,
    app_id: String,
    app_name: String,
}

impl AppQueryParam {
    fn get_r_id(&self) -> &String {
        &self.r_id
    }
    fn get_app_id(&self) -> &String {
        &self.app_id
    }
    fn get_app_name(&self) -> &String {
        &self.app_name
    }
    pub(crate) fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub(crate) fn set_app_id(&mut self, app_id: String) -> &mut Self {
        self.app_id = app_id;
        self
    }
    pub(crate) fn set_app_name(&mut self, app_name: String) -> &mut Self {
        self.app_name = app_name;
        self
    }
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
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
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MenuQueryParam {
    r_id: String,
    menu_id: String,
    menu_title: String,
    menu_parent_id: String,
    menu_path: String,
    app_id: String,
}

impl MenuQueryParam {
    pub(crate) fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub(crate) fn set_menu_id(&mut self, menu_id: String) -> &mut Self {
        self.menu_id = menu_id;
        self
    }
    pub(crate) fn set_menu_title(&mut self, menu_title: String) -> &mut Self {
        self.menu_title = menu_title;
        self
    }
    pub(crate) fn set_menu_path(&mut self, menu_path: String) -> &mut Self {
        self.menu_path = menu_path;
        self
    }
    pub(crate) fn set_app_id(&mut self, app_id: String) -> &mut Self {
        self.app_id = app_id;
        self
    }

    pub(crate) fn get_r_id(&self) -> &String {
        &self.r_id
    }

    pub(crate) fn get_menu_id(&self) -> &String {
        &self.menu_id
    }

    pub(crate) fn get_menu_parent_id(&self) -> &String {
        &self.menu_parent_id
    }

    pub(crate) fn get_menu_title(&self) -> &String {
        &self.menu_title
    }

    pub(crate) fn get_menu_path(&self) -> &String {
        &self.menu_path
    }

    pub(crate) fn get_app_id(&self) -> &String {
        &self.app_id
    }
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

impl ToString for BmbpMenuType {
    fn to_string(&self) -> String {
        match self {
            BmbpMenuType::Route => "route".to_string(),
            BmbpMenuType::URL => "url".to_string(),
            BmbpMenuType::META => "meta".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(dead_code)]
pub struct BmbpMenuVo {
    app_id: String,
    menu_id: String,
    parent_menu_id: String,
    menu_title: String,
    menu_path: String,
    menu_route: String,
    menu_route_type: BmbpMenuType,
    children: Vec<BmbpMenuVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl BmbpMenuVo {
    pub(crate) fn set_app_id(&mut self, app_id: String) -> &mut Self {
        self.app_id = app_id;
        self
    }
    pub(crate) fn set_menu_id(&mut self, menu_id: String) -> &mut Self {
        self.menu_id = menu_id;
        self
    }
    pub(crate) fn set_parent_menu_id(&mut self, parent_menu_id: String) -> &mut Self {
        self.parent_menu_id = parent_menu_id;
        self
    }
    pub(crate) fn set_title(&mut self, menu_title: String) -> &mut Self {
        self.menu_title = menu_title;
        self
    }
    pub(crate) fn set_menu_path(&mut self, menu_path: String) -> &mut Self {
        self.menu_path = menu_path;
        self
    }
    pub(crate) fn set_menu_route(&mut self, menu_route: String) -> &mut Self {
        self.menu_route = menu_route;
        self
    }
    pub(crate) fn set_menu_route_type(&mut self, route_type: BmbpMenuType) -> &mut Self {
        self.menu_route_type = route_type;
        self
    }

    pub(crate) fn get_app_id(&self) -> &String {
        &self.app_id
    }

    pub(crate) fn get_menu_id(&self) -> &String {
        &self.menu_id
    }

    pub(crate) fn get_parent_menu_id(&self) -> &String {
        &self.parent_menu_id
    }
    pub(crate) fn get_menu_title(&self) -> &String {
        &self.menu_title
    }
    pub(crate) fn get_menu_path(&self) -> &String {
        &self.menu_path
    }
    pub(crate) fn get_menu_route(&self) -> &String {
        &self.menu_route
    }
    pub(crate) fn get_menu_route_type(&self) -> &BmbpMenuType {
        &self.menu_route_type
    }
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
        vec![
            "app_id".to_string(),
            "menu_id".to_string(),
            "parent_menu_id".to_string(),
            "menu_title".to_string(),
            "menu_path".to_string(),
            "menu_route".to_string(),
            "menu_route_type".to_string(),
        ]
    }
}
