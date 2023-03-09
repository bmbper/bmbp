use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LoginType {
    USER = 0,
    PHONE = 1,
    FINGER = 2,
    FACE = 3,
    SOCIAL = 4,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginModel {
    // 登录方式
    login_type: LoginType,
    // 用户名
    username: Option<String>,
    // 密码
    password: Option<String>,
    // 验证码
    code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginUserModel {
    username: String,
    token: String,
    user_info: Option<String>,
    org_info: Option<String>,
    role_info: Option<String>,
}
