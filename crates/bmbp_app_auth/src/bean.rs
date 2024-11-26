use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
    pub remember: bool,
    pub captcha: String,
}
