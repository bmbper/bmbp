use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub struct NavMenuParam {
    app_id: String,
}
#[allow(dead_code)]
pub struct LoginParam {}
#[allow(dead_code)]
pub struct MsgParam {}
#[allow(dead_code)]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct NavAppMenuVo {
    title: String,
    route: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<NavAppMenuVo>,
}
