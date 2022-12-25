use tracing::info;

use crate::init_bmbp::{
    init_db_bmbp_struct_from_file, init_db_bmbp_struct_from_rust, load_bmbp_api_from_file,
    load_bmbp_api_from_rust,
};
use crate::load_config::load_bmbp_config;

mod init_bmbp;
mod load_config;
mod load_sys;

pub fn init() {
    info!("0. 初始化应用加载器....");
    // 1. 加载平台配置文件
    let _ = load_bmbp_config();
    // 初始化数据库连接
}

/// bmbp_load 加载平台配置
pub fn bmbp_load() -> Result<String, String> {
    init();
    // 2. 创建平台数据库表结构
    let _ = init_bmbp_default_struct();
    // 3. 初始化平台接口
    let _ = init_bmbp_default_api();
    // 4. 初始化业务表结构
    let _ = init_bmbp_bus_struct();
    // 5. 初始化业务接口
    let _ = init_bmbp_bus_api();
    // 6. 初始化业务数据
    let _ = init_bmbp_bus_data();
    // 7. 读取平台配置
    let _ = init_bmbp_setting();
    // 8. 初始化平台组件
    let _ = init_bmbp_component();
    Ok("平台加载完成".to_string())
}

fn init_bmbp_default_struct() -> Result<String, String> {
    info!("init_2.{}", "初始化平台数据表结构...");
    // 从代码获取表结构配置
    let _ = init_db_bmbp_struct_from_rust();
    // 从文件获取表结构配置
    let _ = init_db_bmbp_struct_from_file();
    Ok("".to_string())
}

/// init_bmbp_default_api
fn init_bmbp_default_api() -> Result<String, String> {
    info!("init_3.{}", "初始化平台API...");
    let _ = load_bmbp_api_from_rust();
    let _ = load_bmbp_api_from_file();
    Ok("".to_string())
}

/// init_bmbp_bus_struct
fn init_bmbp_bus_struct() -> Result<String, String> {
    info!("init_4.{}", "初始化业务表结构...");

    Ok("".to_string())
}

/// init_bmbp_bus_api
fn init_bmbp_bus_api() -> Result<String, String> {
    info!("init_5.{}", "初始化业务API...");

    Ok("".to_string())
}

/// init_bmbp_bus_data
fn init_bmbp_bus_data() -> Result<String, String> {
    info!("init_6.{}", "初始化应业务数据...");

    Ok("".to_string())
}

fn init_bmbp_component() -> Result<String, String> {
    info!("8.{}", "加载组件....");

    let _ = load_tec_comp();
    let _ = load_bus_comp();
    Ok("组件加载成功".to_string())
}

fn load_tec_comp() -> Result<String, String> {
    info!("8.1.{}", "加载技术组件....");
    Ok("9.加载技术组件....".to_string())
}

fn load_bus_comp() -> Result<String, String> {
    info!("8.2.{}", "加载业务组件....");
    Ok("10.加载业务组件....".to_string())
}

/// 加载系统参数缓存
pub fn init_bmbp_setting() -> Result<String, String> {
    info!("7.{}", "读取系统配置....");
    Ok("8.读取系统配置....".to_string())
}

#[cfg(test)]
mod tests {
    use crate::bmbp_load;

    #[test]
    fn test_bmbp_load() {
        let res = bmbp_load();
        match res {
            Ok(v) => {
                println!("{}", v);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        assert_eq!(2 + 2, 4);
    }
}
