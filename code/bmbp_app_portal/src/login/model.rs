use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LoginType {
    USER = 0,
    PHONE = 1,
    FINGER = 2,
    FACE = 3,
    SOCIAL = 4,
}

impl Default for LoginType {
    fn default() -> Self {
        LoginType::USER
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct LoginUser {
    // 登录方式
    login_type: LoginType,
    // 用户名
    username: Option<String>,
    // 密码
    password: Option<String>,
    // 验证码
    code: Option<String>,
}

impl LoginUser {
    pub fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpUserInfo {
    username: Option<String>,
    token: Option<String>,
    user_info: Option<String>,
    org_info: Option<String>,
    role_info: Option<String>,
}

impl BmbpUserInfo {
    pub fn set_username(&mut self, user_name: String) -> &mut Self {
        self.username = Some(user_name);
        self
    }
    pub fn set_token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }
}
