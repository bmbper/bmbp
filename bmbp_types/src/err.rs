use axum::response::IntoResponse;

#[derive(Debug)]
pub enum BmbpErrorKind {
    EMPTY,
    ORM,
}

impl ToString for BmbpErrorKind {
    fn to_string(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
        }
    }
}

impl BmbpErrorKind {
    pub fn name(&self) -> String {
        match self {
            BmbpErrorKind::EMPTY => "".to_string(),
            BmbpErrorKind::ORM => "ORM".to_string(),
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
            msg: msg,
        }
    }
}

impl ToString for BmbpError {
    fn to_string(&self) -> String {
        format!("{}:{}", self.name(), self.msg)
    }
}

impl IntoResponse for BmbpError {
    fn into_response(self) -> axum::response::Response {
        self.to_string().into_response()
    }
}

pub type BmbpResp<T> = Result<T, BmbpError>;
