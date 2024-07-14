use ::serde::Deserialize;
use ::serde::Serialize;
use chrono::Utc;
use salvo::*;
use tracing::*;

use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;

#[rdbc_model(BMBP_DEV_DATASOURCE)]
pub struct BmbpDevDatasource {
    // 数据源名称
    datasource_name: Option<String>,
    // 数据源类型
    datasource_type: Option<String>,
    // 数据源主机
    datasource_host: Option<String>,
    // 数据源端口
    datasource_port: Option<i32>,
    // 数据源用户名
    datasource_username: Option<String>,
    // 数据源密码
    datasource_password: Option<String>,
    // 数据源模式
    datasource_schema: Option<String>,
    // 连接参数
    datasource_url_params: Option<String>,
    // 数据源描述
    datasource_desc: Option<String>,
    // 所属应用
    owner_app_id: Option<String>,
}

/// BmbpDevTable 存储表结构描述
#[rdbc_model(BMBP_DEV_TABLE)]
pub struct BmbpDevTable {
    // 数据源ID
    datasource_id: Option<String>,
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 所属模式
    owner_schema: Option<String>,
    // 数据表名称
    table_name: Option<String>,
    // 数据表注释
    table_comment: Option<String>,
    // 数据表描述
    table_desc: Option<String>,
    // 数据表配置文件
    table_meta: Option<String>,
}
