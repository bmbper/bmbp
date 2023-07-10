use crate::RespVo;

#[derive(Debug)]
pub enum BmbpErrorKind {
    EMPTY,
    ORM,
    DynSQL,
    ApiService,
    Model,
}

impl ToString for BmbpErrorKind {
    fn to_string(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
            BmbpErrorKind::ApiService => "apiService".to_string(),
            BmbpErrorKind::DynSQL => "DynSql".to_string(),
            BmbpErrorKind::Model => "Model".to_string(),
        }
    }
}

impl BmbpErrorKind {
    pub fn name(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
            BmbpErrorKind::ApiService => "ApiService".to_string(),
            BmbpErrorKind::DynSQL => "DynSQL".to_string(),
            BmbpErrorKind::Model => "Model".to_string(),
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
    pub fn api(msg: String) -> Self {
        BmbpError {
            kind: BmbpErrorKind::ApiService,
            msg,
        }
    }
    pub fn dyn_sql(msg: String) -> Self {
        BmbpError {
            kind: BmbpErrorKind::DynSQL,
            msg,
        }
    }
    pub fn valid(msg: &str) -> Self {
        BmbpError {
            kind: BmbpErrorKind::Model,
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
