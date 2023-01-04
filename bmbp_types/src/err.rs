use axum::response::IntoResponse;

use crate::vo::BmbpApiResponseVo;
use crate::RespVo;

#[derive(Debug)]
pub enum BmbpErrorKind {
    EMPTY,
    ORM,
    ApiService,
}

impl ToString for BmbpErrorKind {
    fn to_string(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
            BmbpErrorKind::ApiService => "apiService".to_string(),
        }
    }
}

impl BmbpErrorKind {
    pub fn name(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
            BmbpErrorKind::ApiService => "ApiService".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct BmbpError {
    kind: BmbpErrorKind,
    msg: String,
}

impl BmbpError {
    pub fn name(&self) -> String {
        self.kind.name()
    }

    pub fn orm(msg: String) -> Self {
        BmbpError {
            kind: BmbpErrorKind::ORM,
            msg,
        }
    }

    pub fn api_service(msg: String) -> Self {
        BmbpError {
            kind: BmbpErrorKind::ApiService,
            msg,
        }
    }
}

impl ToString for BmbpError {
    fn to_string(&self) -> String {
        let vo: BmbpApiResponseVo<String> =
            RespVo::fail_msg(format!("{}:{}", self.name(), self.msg));
        serde_json::to_string(&vo).unwrap()
    }
}

impl IntoResponse for BmbpError {
    fn into_response(self) -> axum::response::Response {
        let vo: BmbpApiResponseVo<String> =
            RespVo::fail_msg(format!("{}:{}", self.name(), self.msg));
        vo.into_response()
    }
}

pub type BmbpResp<T> = Result<T, BmbpError>;
