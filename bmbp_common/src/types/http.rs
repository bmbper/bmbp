use serde::{Deserialize, Serialize};

/// BmbpHttp<T>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpHttp<T> {
    pub code: i8,
    pub msg: String,
    pub data: Option<T>,
}
