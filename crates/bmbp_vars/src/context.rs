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
        .unwrap_or(&"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".to_string())
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
