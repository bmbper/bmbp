use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BmbpFileInfo {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
}
