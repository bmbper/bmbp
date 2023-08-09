use super::model::*;
use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpResp;
use bmbp_app_utils::simple_uuid_upper;

pub struct LoginService;
impl LoginService {
    pub async fn do_login(login_user: &LoginUser) -> BmbpResp<BmbpUserInfo> {
        let username = {
            match login_user.get_username() {
                Some(v) => v.to_string(),
                None => "".to_string(),
            }
        };
        let password = {
            match login_user.get_password() {
                Some(v) => v.to_string(),
                None => "".to_string(),
            }
        };
        // TODO 替换成实际验证
        if (!username.eq("zhangguokai")) || (!password.eq("zhangguokai")) {
            return Err(BmbpError::api("用户名或密码不正确"));
        }

        let mut user_info = BmbpUserInfo::default();
        user_info.set_username(username);
        user_info.set_token(simple_uuid_upper());
        Ok(user_info)
    }
}
