use bmbp_bean::RespVo;
use bmbp_orm::error::OrmError;
use salvo::prelude::Text::Plain;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpError {
    pub kind: BmbpErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpErrorKind {
    NotFound { code: String, msg: String },
    Auth { code: String, msg: String },
    Valid { code: String, msg: String },
    Api { code: String, msg: String },
    ORM { code: String, msg: String },
    Other { code: String, msg: String },
}
impl Default for BmbpErrorKind {
    fn default() -> Self {
        BmbpErrorKind::Other {
            code: "-1".to_string(),
            msg: "操作异常".to_string(),
        }
    }
}

impl BmbpErrorKind {
    pub fn code_msg(&self) -> (String, String) {
        match self {
            BmbpErrorKind::NotFound { code, msg } => (code.clone(), msg.clone()),
            BmbpErrorKind::Auth { code, msg } => (code.clone(), msg.clone()),
            BmbpErrorKind::Valid { code, msg } => (code.clone(), msg.clone()),
            BmbpErrorKind::Api { code, msg } => (code.clone(), msg.clone()),
            BmbpErrorKind::ORM { code, msg } => (code.clone(), msg.clone()),
            BmbpErrorKind::Other { code, msg } => (code.clone(), msg.clone()),
        }
    }
}

impl BmbpError {
    pub fn orm(msg: String) -> BmbpError {
        BmbpError {
            kind: BmbpErrorKind::ORM {
                code: "1000".to_string(),
                msg: "ORM异常".to_string(),
            },
            message: msg,
        }
    }
    pub fn valid(msg: &str) -> BmbpError {
        BmbpError {
            kind: BmbpErrorKind::Valid {
                code: "1000".to_string(),
                msg: "校验失败".to_string(),
            },
            message: msg.to_string(),
        }
    }
}

pub type BmbpResp<T> = Result<T, BmbpError>;

#[async_trait]
impl Writer for BmbpError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let mut resp = RespVo::<String> {
            code: "0".to_string(),
            msg: "".to_string(),
            data: None,
        };
        let error_kind = self.kind.code_msg();
        resp.code = error_kind.0;
        resp.msg = format!("{}:{}", error_kind.1, self.message);
        let resp_string = serde_json::to_string(&resp)
            .unwrap_or("{\"code\":\"5000\",\"msg\":\"异常转换借误\"}".to_string());
        res.render(Plain(resp_string));
    }
}
impl From<salvo::Error> for BmbpError {
    fn from(value: salvo::Error) -> Self {
        BmbpError {
            kind: BmbpErrorKind::Api {
                code: "0000".to_string(),
                msg: "API".to_string(),
            },
            message: value.to_string(),
        }
    }
}

impl From<OrmError> for BmbpError {
    fn from(value: OrmError) -> Self {
        BmbpError {
            kind: BmbpErrorKind::ORM {
                code: "1000".to_string(),
                msg: "ORM异常".to_string(),
            },
            message: value.msg,
        }
    }
}
