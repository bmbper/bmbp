use ::serde::Deserialize;
use ::serde::Serialize;
use chrono::Utc;
use salvo::*;
use tracing::*;

use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;

/// BmbpDevTable 存储表结构描述
#[rdbc_model(BMBP_DEV_REPORT)]
pub struct BmbpDevReport {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 报表名称
    report_name: Option<String>,
    // 报表描述
    report_desc: Option<String>,
    // 报表缩略图
    report_img: Option<String>,
    // 报表配置
    report_meta: Option<String>,
}
