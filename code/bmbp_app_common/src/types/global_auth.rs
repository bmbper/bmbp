use serde::Deserialize;
use serde::Serialize;

use crate::BmbpHashMap;
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpAuthInfo {
    user_id: String,
    user_name: String,
    user_nick_name: String,
    user_person_code: Option<String>,
    user_org_code: String,
    user_role: Vec<BmbpHashMap>,
    user_api: Vec<BmbpHashMap>,
    user_data: Vec<BmbpHashMap>,
    user_menu: Vec<BmbpHashMap>,
}

impl BmbpAuthInfo {
    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }
    pub fn get_user_name(&self) -> &String {
        &self.user_name
    }
    pub fn get_user_nick_name(&self) -> &String {
        &self.user_nick_name
    }
    pub fn get_user_org_code(&self) -> &String {
        &self.user_org_code
    }
    pub fn get_user_person_code(&self) -> Option<&String> {
        self.user_person_code.as_ref()
    }
}
