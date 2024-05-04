use bmbp_rdbc_marco::rdbc_model;
use serde::{Deserialize, Serialize};

use crate::app::model::types::RbacAppType;

#[rdbc_model]
pub struct BmbpRbacApp {
    // 应用编码
    app_code: Option<String>,
    // 应用名称
    app_name: Option<String>,
    // 应用简称
    app_short_name: Option<String>,
    // 应用图标
    app_icon: Option<String>,
    // 应用描述
    app_desc: Option<String>,
    // 应用类型
    app_type: Option<RbacAppType>,
    // 应用主题
    app_theme: Option<String>,
    // 应用地址
    app_url: Option<String>,
}
