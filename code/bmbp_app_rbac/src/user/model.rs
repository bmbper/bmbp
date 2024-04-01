use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacUser {
    organ_id: Option<String>,
    name: Option<String>,
    nick_name: Option<String>,
    password: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
}

impl BmbpRbacUser {}
