use bmbp_app_common::BmbpHashMap;

pub fn is_empty_map(map: &BmbpHashMap) -> bool {
    map.len() == 0
}

pub fn is_empty_prop(map: &BmbpHashMap, key: &str) -> bool {
    if is_empty_map(map) {
        return true;
    }

    let val = map.get(key);
    match val {
        Some(v) => match v {
            bmbp_app_common::BmbpValue::String(v) => v.is_empty(),
            bmbp_app_common::BmbpValue::Map(v) => v.len() == 0,
            bmbp_app_common::BmbpValue::Array(v) => v.len() == 0,
            bmbp_app_common::BmbpValue::NULL => true,
            _ => false,
        },
        None => true,
    }
}
