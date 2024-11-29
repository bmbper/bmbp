use serde::{Deserialize, Serialize};

pub trait BmbpBaseModel {
    fn base_mut(&mut self) -> &mut BmbpBase;
    fn base(&self) -> &BmbpBase;
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpBase {
    pub data_id: String,
    pub data_status: String,
    pub data_flag: String,
    pub data_level: String,
    pub data_sign: String,
    pub data_owner_org_code: String,
    pub data_create_time: String,
    pub data_create_user: String,
    pub data_update_time: String,
    pub data_update_user: String,
}
