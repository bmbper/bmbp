use serde::{Deserialize, Serialize};

use bmbp_types::vo::BaseOrmModel;
use bmbp_types::{BmbpBaseModel, BmbpPageReqVo};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserQueryParam {
    r_id: String,
    user_code: String,
    user_name: String,
}
#[allow(dead_code)]
pub type UserPageQueryParam = BmbpPageReqVo<UserQueryParam>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpUserVo {
    // 登录账号
    user_code: String,

    // 登录账户姓名
    user_name: String,

    // 用户密码
    user_pass: String,
    // 密码修改时间
    user_pass_change_time: String,
    // 用户电话
    user_phone: String,

    //  用户邮箱
    user_email: String,

    // 所属组织
    user_organ: String,

    #[serde(flatten)]
    base: BmbpBaseModel,
}

impl BaseOrmModel for BmbpUserVo {
    fn get_base_vo(&self) -> &BmbpBaseModel {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BmbpBaseModel) -> &mut Self {
        self.base = vo;
        self
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}
