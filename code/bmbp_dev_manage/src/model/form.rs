use ::serde::Deserialize;
use ::serde::Serialize;
use chrono::Utc;
use salvo::*;
use tracing::*;

use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;

/// BmbpDevForm 保存表低代码表单
#[rdbc_model(BMBP_DEV_FORM)]
pub struct BmbpDevForm {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 流程名称
    form_name: Option<String>,
    // 流程描述
    form_desc: Option<String>,
    // 流程缩略图
    form_img: Option<String>,
    // 流程配置
    form_meta: Option<String>,
}
