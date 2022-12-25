use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

/// GLOBAL_HASH_MAP_VARS 二级参数集合
pub static GLOBAL_HASH_MAP_VARS: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> =
    Lazy::new(|| {
        let cache: HashMap<String, HashMap<String, String>> = HashMap::new();
        RwLock::new(cache)
    });

pub fn set_global_hash_map_vars(map_name: String, var_name: String, var_value: String) {
    let mut map_write_lock = GLOBAL_HASH_MAP_VARS.write().unwrap();
    let write_map = map_write_lock.get_mut(map_name.as_str());
    match write_map {
        None => {
            let mut new_map = HashMap::new();
            new_map.insert(var_name, var_value);
            map_write_lock.insert(map_name, new_map);
        }
        Some(mp) => {
            mp.insert(var_name, var_value);
        }
    }
}

pub fn global_hash_map_vars(map_name: String, var_name: String) -> String {
    let map_read_lock = GLOBAL_HASH_MAP_VARS.read().unwrap();
    match map_read_lock.get(map_name.as_str()) {
        None => "".to_string(),
        Some(map_vars) => match map_vars.get(var_name.as_str()) {
            None => "".to_string(),
            Some(v) => v.to_string(),
        },
    }
}

pub fn global_hash_map_vars_to_usize(map_name: String, var_name: String) -> usize {
    let map_read_lock = GLOBAL_HASH_MAP_VARS.read().unwrap();
    match map_read_lock.get(map_name.as_str()) {
        None => 0usize,
        Some(map_vars) => match map_vars.get(var_name.as_str()) {
            None => 0usize,
            Some(v) => {
                let u_v = v.parse::<usize>();
                match u_v {
                    Ok(u_v_v) => u_v_v.clone(),
                    Err(_) => 0usize,
                }
            }
        },
    }
}

pub fn global_hash_map_vars_to_bool(map_name: String, var_name: String) -> bool {
    let map_read_lock = GLOBAL_HASH_MAP_VARS.read().unwrap();
    match map_read_lock.get(map_name.as_str()) {
        None => false,
        Some(map_vars) => match map_vars.get(var_name.as_str()) {
            None => false,
            Some(v) => {
                let u_v = v.parse::<bool>();
                match u_v {
                    Ok(u_v_v) => u_v_v.clone(),
                    Err(_) => false,
                }
            }
        },
    }
}
