use serde::{Deserialize, Serialize};

use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, PageReqVo};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MenuQueryParam {
    r_id: String,
    user_code: String,
    user_name: String,
}

pub type MenuPageQueryParam = PageReqVo<MenuQueryParam>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpMenuVo {}
