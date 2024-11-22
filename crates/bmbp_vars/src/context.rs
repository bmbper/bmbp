use crate::{
    BMBP_APP_COPY_WRITE, BMBP_APP_EMAIL, BMBP_APP_GROUP_NAME, BMBP_APP_ICON, BMBP_APP_LOCALE,
    BMBP_APP_LOGIN_NAME, BMBP_APP_MODEL, BMBP_APP_NAME, BMBP_APP_SHORT_NAME, BMBP_APP_TITLE,
    BMBP_APP_VERSION,
};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub static BMBP_CONTEXT_VARS: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn app_title() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_TITLE)
        .unwrap_or(&"".to_string())
        .to_string()
}
pub fn app_icon() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_ICON)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_name() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_NAME)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_short_name() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_SHORT_NAME)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_login_name() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_LOGIN_NAME)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_group_name() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_GROUP_NAME)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_copy_right() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_COPY_WRITE)
        .unwrap_or(&"".to_string())
        .to_string()
}
pub fn app_version() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_VERSION)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_email() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_EMAIL)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_model() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_MODEL)
        .unwrap_or(&"".to_string())
        .to_string()
}

pub fn app_locale() -> String {
    BMBP_CONTEXT_VARS
        .read()
        .unwrap()
        .get(BMBP_APP_LOCALE)
        .unwrap_or(&"zh_cn".to_string())
        .to_string()
}
