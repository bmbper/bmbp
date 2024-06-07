use ::serde::Deserialize;
use ::serde::Serialize;
use chrono::Utc;
use salvo::*;
use tracing::*;

use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;

/// BmbpDevhttp 接口类型
#[rdbc_model(BMBP_DEV_HTTP)]
pub struct BmbpDevHttp {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 流程名称
    http_name: Option<String>,
    http_method: Option<HttpMethod>,
    // 流程描述
    http_desc: Option<String>,
    // 流程配置
    http_meta: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum HttpMethod {
    #[default]
    POST,
    GET,
    DELETE,
    PUT,
}
impl From<&HttpMethod> for RdbcValue {
    fn from(value: &HttpMethod) -> Self {
        match value {
            HttpMethod::POST => RdbcValue::String("POST".to_string()),
            HttpMethod::GET => RdbcValue::String("GET".to_string()),
            HttpMethod::DELETE => RdbcValue::String("DELETE".to_string()),
            HttpMethod::PUT => RdbcValue::String("PUT".to_string()),
        }
    }
}

impl Into<HttpMethod> for &RdbcValue {
    fn into(self) -> HttpMethod {
        let meta = self.get_string();
        if meta == "POST" {
            return HttpMethod::POST;
        } else if meta == "GET" {
            return HttpMethod::GET;
        } else if meta == "DELETE" {
            return HttpMethod::DELETE;
        } else if meta == "PUT" {
            return HttpMethod::PUT;
        }
        HttpMethod::POST
    }
}
