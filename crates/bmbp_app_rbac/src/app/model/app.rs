use crate::app::model::types::RbacAppType;
use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_app_orm::*;
use bmbp_marco::rdbc_model;
use chrono::Utc;
use salvo::*;
use tracing::*;
#[rdbc_model(table = BMBP_RBAC_APP)]
pub struct BmbpRbacApp {
    // 应用编码
    app_code: Option<String>,
    // 应用名称
    #[query(like)]
    #[valid[require("应用名称不能为空"),unique("应用名称不能不能重复"),max_length(50,"应用名称不能超过50个字符"),min_length(2,"应用名称不能少于2个字符")]]
    app_name: Option<String>,
    // 应用简称
    #[query(like)]
    app_short_name: Option<String>,
    // 应用图标
    app_icon: Option<String>,
    // 应用描述
    app_desc: Option<String>,
    // 应用类型
    #[query(eq)]
    app_type: Option<RbacAppType>,
    // 应用主题
    app_theme: Option<String>,
    // 应用地址
    app_url: Option<String>,
}
