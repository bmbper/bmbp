use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

/// BMBP_CACHE 平台配置缓存
pub static GLOBAL_APP_VARS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let cache: HashMap<String, String> = HashMap::new();
    RwLock::new(cache)
});

pub fn set_global_vars(var_name: String, var_value: String) {
    GLOBAL_APP_VARS.write().unwrap().insert(var_name, var_value);
}

pub fn global_vars(var_name: String) -> String {
    let vars_value_lock = GLOBAL_APP_VARS.read().unwrap();
    let vars_value = vars_value_lock.get(var_name.as_str());
    match vars_value {
        Some(value) => value.clone(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_global_vars() {}
}
