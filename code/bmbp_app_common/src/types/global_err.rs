use salvo::{http::ParseError, writing::Json, Piece};

use crate::RespVo;

#[derive(Debug)]
pub enum BmbpErrorKind {
    API,
    SERVICE,
    DAO,
    SCRIPT,
    ORM,
    UTIL,
    OTHER,
}

impl ToString for BmbpErrorKind {
    fn to_string(&self) -> String {
        match self {
            BmbpErrorKind::API => "API".to_string(),
            BmbpErrorKind::SERVICE => "SERVICE".to_string(),
            BmbpErrorKind::DAO => "DAO".to_string(),
            BmbpErrorKind::SCRIPT => "SCRIPT".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
            BmbpErrorKind::UTIL => "UTIL".to_string(),
            BmbpErrorKind::OTHER => "OTHER".to_string(),
        }
    }
}

impl BmbpErrorKind {
    pub fn name(&self) -> String {
        self.to_string()
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
    pub fn api(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::API,
            msg: msg.to_string(),
        }
    }
    pub fn service(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::SERVICE,
            msg: msg.to_string(),
        }
    }
    pub fn dao(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::DAO,
            msg: msg.to_string(),
        }
    }
    pub fn script(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::SCRIPT,
            msg: msg.to_string(),
        }
    }
    pub fn orm(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::ORM,
            msg: msg.to_string(),
        }
    }
    pub fn util(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::UTIL,
            msg: msg.to_string(),
        }
    }
    pub fn other(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::OTHER,
            msg: msg.to_string(),
        }
    }
}

impl ToString for BmbpError {
    fn to_string(&self) -> String {
        let vo: RespVo<String> = RespVo::fail_msg(format!("{}:{}", self.name(), self.msg).as_str());
        serde_json::to_string(&vo).unwrap()
    }
}

impl Piece for BmbpError {
    fn render(self, res: &mut salvo::Response) {
        let vo: RespVo<String> = RespVo::fail_msg(format!("{}:{}", self.name(), self.msg).as_str());
        res.render(Json(vo));
    }
}

impl From<ParseError> for BmbpError {
    fn from(value: ParseError) -> Self {
        BmbpError {
            kind: BmbpErrorKind::API,
            msg: value.to_string(),
        }
    }
}
