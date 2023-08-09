use serde::{Deserialize, Serialize};

/// BMBP配置信息
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Bmbp {
    pub bmbp: Option<BmbpItem>,
}

impl Bmbp {
    pub fn bmbp(&self) -> Option<&BmbpItem> {
        self.bmbp.as_ref()
    }
}

/// BMBP配置信息
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpItem {
    app: Option<BmbpItemApp>,
    db: Option<BmbpItemDb>,
}

impl BmbpItem {
    pub fn app(&self) -> Option<&BmbpItemApp> {
        self.app.as_ref()
    }
    pub fn db(&self) -> Option<&BmbpItemDb> {
        self.db.as_ref()
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpItemApp {
    app_code: Option<String>,
    app_name: Option<String>,
    app_title: Option<String>,
    app_login: Option<String>,
    app_copy_right: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpItemDb {
    driver: Option<String>,
    pub host: Option<String>,
    port: Option<String>,
    schema: Option<String>,
    user_name: Option<String>,
    pass_word: Option<String>,
    max_connection: Option<usize>,
    idle_connection: Option<usize>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpSettingItem {
    group: String,
    code: String,
    name: String,
    title: String,
    value: String,
    is_dynamic: bool,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpCombo {
    group: String,
    items: Vec<BmbpSettingItem>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpComboItem {
    code: String,
    name: String,
    title: String,
    group: String,
}
