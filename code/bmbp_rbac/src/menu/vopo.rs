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
    menu_route_type: Option<BmbpMenuRouteType>,
    menu_path: String,
    menu_type: Option<BmbpMenuType>,
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
    pub(crate) fn set_menu_route_type(&mut self, menu_route_type: BmbpMenuRouteType) -> &mut Self {
        self.menu_route_type = Some(menu_route_type);
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

    pub(crate) fn get_menu_route_type(&self) -> Option<&BmbpMenuRouteType> {
        self.menu_route_type.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpMenuRouteType {
    ROUTE, // 本地应用路由
    URL,   // 接入外部页面地址
    META,  // 配置界面
}

impl ToString for BmbpMenuRouteType {
    fn to_string(&self) -> String {
        match self {
            BmbpMenuRouteType::ROUTE => "ROUTE".to_string(),
            BmbpMenuRouteType::URL => "URL".to_string(),
            BmbpMenuRouteType::META => "META".to_string(),
        }
    }
}
impl Default for BmbpMenuRouteType {
    fn default() -> Self {
        BmbpMenuRouteType::ROUTE
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpMenuType {
    FUNC,
    MODULE,
}
impl ToString for BmbpMenuType {
    fn to_string(&self) -> String {
        match self {
            BmbpMenuType::FUNC => "FUNC".to_string(),
            BmbpMenuType::MODULE => "MODULE".to_string(),
        }
    }
}
impl Default for BmbpMenuType {
    fn default() -> Self {
        BmbpMenuType::FUNC
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
    menu_route_type: Option<BmbpMenuRouteType>,
    menu_type: BmbpMenuType,
    menu_order: u32,
    children: Vec<BmbpMenuVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}
#[allow(dead_code)]
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
    pub(crate) fn set_menu_route_type(&mut self, route_type: BmbpMenuRouteType) -> &mut Self {
        self.menu_route_type = Some(route_type);
        self
    }
    pub(crate) fn set_menu_type(&mut self, menu_type: BmbpMenuType) -> &mut Self {
        self.menu_type = menu_type;
        self
    }
    pub(crate) fn set_menu_order(&mut self, menu_order: u32) -> &mut Self {
        self.menu_order = menu_order;
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
    pub(crate) fn get_menu_route_type(&self) -> Option<&BmbpMenuRouteType> {
        self.menu_route_type.as_ref()
    }
    pub(crate) fn get_menu_order(&self) -> &u32 {
        &self.menu_order
    }

    pub(crate) fn get_menu_type(&self) -> &BmbpMenuType {
        &self.menu_type
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
            "menu_type".to_string(),
            "menu_route".to_string(),
            "menu_route_type".to_string(),
            "menu_order".to_string(),
        ]
    }
}
