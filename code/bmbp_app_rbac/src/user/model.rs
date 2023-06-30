use bmbp_app_common::BmbpBaseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserParams {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacUser {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 所属组织
    organ_code: Option<String>,
    /// 关联人员
    person_code: Option<String>,
    /// 登录账号
    user_name: Option<String>,
    /// 用户昵称
    nick_name: Option<String>,
    /// 用户密码
    user_pass: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RbacUserExtend {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 是否首次登录
    is_first_login: Option<String>,
    /// 最近一次登录时间
    last_login_time: Option<String>,
    /// 最近一次登录IP
    last_login_ip: Option<String>,
    /// 最近一次登录类型
    last_login_type: Option<String>,
    /// 用户密码最后修改时间
    last_password_change_time: Option<String>,
    /// 前一次密码
    history_password_one: Option<String>,
    /// 前二次所用密码
    history_password_two: Option<String>,
    /// 前三次所有密码
    history_password_three: Option<String>,
}
