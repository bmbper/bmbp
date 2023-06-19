use std::fs;

use toml::Value;

use bmbp_app_common::app::set_global_vars;
use bmbp_app_common::map::set_global_hash_map_vars;

pub fn load_config_to_global_vars() {
    tracing::info!("加配配置文件到全局变量中......");
    let config_path = "../config/bmbp.toml";
    let bmbp_config_rs = fs::read_to_string(config_path);
    match bmbp_config_rs {
        Ok(bmbp_config_str) => {
            let bmbp_entity_rs = toml::from_str::<Value>(bmbp_config_str.as_str());
            match bmbp_entity_rs {
                Ok(bmbp_entity) => parse_bmbp_config(&bmbp_entity),
                Err(e) => {
                    tracing::error!("{}", e.to_string());
                    set_need_init_vars();
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            set_need_init_vars();
        }
    }
}

fn parse_bmbp_config(bmbp_config: &Value) {
    tracing::info!("解析配置文件信息...");
    // bmbp-app-key
    // bmbp-server-host
    match bmbp_config {
        Value::Table(bmbp_config_item) => {
            // bmbp
            let root_key = "bmbp";
            if let Some(bmbp_config_item_value) = bmbp_config_item.get(root_key) {
                // app
                // server
                // datasource
                match bmbp_config_item_value {
                    Value::Table(bmbp_config_first_item) => {
                        for first_key in bmbp_config_first_item.keys() {
                            // bmbp_app
                            // bmbp_server
                            // bmbp_ds
                            let vars_map_key_name = format!("{}_{}", root_key, first_key);
                            let bmbp_config_second_item =
                                bmbp_config_first_item.get(first_key).unwrap();
                            // db_host
                            match bmbp_config_second_item {
                                Value::Table(bmbp_config_second_item_value) => {
                                    for second_item_key in bmbp_config_second_item_value.keys() {
                                        let vars_key_name =
                                            format!("{}_{}", first_key, second_item_key);
                                        let third_item_value = bmbp_config_second_item_value
                                            .get(second_item_key)
                                            .unwrap();
                                        let value_string = to_string_value(third_item_value);
                                        set_global_hash_map_vars(
                                            vars_map_key_name.clone(),
                                            vars_key_name,
                                            value_string,
                                        );
                                    }
                                }
                                _ => {
                                    tracing::error!("配置文件bmbp二级属性配置错误");
                                    set_need_init_vars()
                                }
                            }
                        }
                    }
                    _ => {
                        tracing::error!("配置文件bmbp属性配置错误...");
                        set_need_init_vars()
                    }
                }
            } else {
                set_need_init_vars();
            }
        }
        _ => {
            tracing::info!("解析配置文件信息...");
        }
    }
}

fn set_need_init_vars() {
    tracing::error!("读取配置文件失败,转向初始化页面...");
    set_global_vars("need_init".to_string(), "true".to_string())
}

fn to_string_value(value: &Value) -> String {
    match value {
        Value::String(v) => v.clone(),
        Value::Integer(v) => {
            format!("{}", v)
        }
        Value::Float(v) => {
            format!("{}", v)
        }
        Value::Boolean(v) => {
            format!("{}", v)
        }
        Value::Datetime(v) => {
            format!("{}", v)
        }
        _ => "".to_string(),
    }
}
