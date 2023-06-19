use std::collections::HashMap;

/// init_url_query
/// 将 name=33&&code=333 解析成参数MAP
/// name=2&3&&code=33&&&&&&&go=g&&d=dd
pub fn init_url_query(raw_url_query: &str) -> Result<HashMap<String, String>, String> {
    let mut query_cache = HashMap::new();
    let raw_query_key_value_array = raw_url_query.split("&");
    for raw_query_key_value in raw_query_key_value_array {
        // 忽略空值： 多个&拼接产生的参数空值
        if raw_query_key_value.is_empty() {
            continue;
        }
        // SQL语句 无法识別;
        if raw_query_key_value.contains(";") {
            return Err(raw_query_key_value.to_string() + ":非法的URL参数");
        }

        // 参数对 以 = 分隔
        let mut key_value = raw_query_key_value.split("=");
        let size = key_value.clone().count();
        if size == 2 {
            let key = key_value.next().unwrap_or("");
            let value = key_value.next().unwrap_or("");
            query_cache.insert(key.to_string(), value.to_string());
        } else {
            return Err(raw_query_key_value.to_string() + ":非法的URL参数");
        }
    }
    Ok(query_cache)
}

#[cfg(test)]
mod tests {
    use crate::url::init_url_query;

    #[test]
    fn init_url_query_test() {
        let st = "name=2&3=4&&code=3;3&&&&&&&go=g&&d=d;d";
        let mp = init_url_query(st);
        println!("{:?}", mp);
    }
}
