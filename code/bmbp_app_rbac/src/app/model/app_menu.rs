use crate::app::model::types::{RbacAppMenuOpenType, RbacAppMenuType};
use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::RdbcMacroTree;
use bmbp_rdbc_orm::*;
use chrono::Utc;
use salvo::*;
use tracing::info;
use uuid::Uuid;
#[rdbc_model(BMBP_RBAC_APP_MENU, MENU)]
pub struct BmbpRbacAppMenu {
    // 应用ID
    #[query(eq)]
    app_id: Option<String>,
    // 菜单简称
    #[query(like)]
    menu_short_name: Option<String>,
    // 菜单类型
    menu_type: Option<RbacAppMenuType>,
    // 菜单配置类型
    menu_meta_type: Option<String>,
    // 打开类型
    menu_open_type: Option<RbacAppMenuOpenType>,
    // 菜单图标
    menu_icon: Option<String>,
    // 菜单地址
    menu_url: Option<String>,
    // 菜单说明
    menu_desc: Option<String>,
}
