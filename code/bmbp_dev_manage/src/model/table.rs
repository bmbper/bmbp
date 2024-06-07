use ::serde::Deserialize;
use ::serde::Serialize;
use chrono::Utc;
use salvo::*;
use tracing::*;

use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;

/// BmbpDevTable 存储表结构描述
#[rdbc_model(BMBP_DEV_TABLE)]
pub struct BmbpDevTable {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 数据源ID
    datasource_id: Option<String>,
    // 数据表名称
    table_name: Option<String>,
    // 数据表注释
    table_comment: Option<String>,
    // 数据表描述
    table_desc: Option<String>,
    // 数据表配置文件
    table_meta: Option<String>,
}
