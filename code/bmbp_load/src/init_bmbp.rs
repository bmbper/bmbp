use tracing::info;

///  init_db_bmbp_struct 初始化平台表结构
pub fn init_db_bmbp_struct_from_rust() -> Result<String, String> {
    info!("init_2.1.{}", "从代码中_初始化平台表结构....");
    Ok("".to_string())
}

///  init_db_bmbp_struct 初始化平台表结构
pub fn init_db_bmbp_struct_from_file() -> Result<String, String> {
    info!("init_2.2.{}", "从文件中_初始化平台表结构....");
    info!("init_2.2.1 {}", "读取[init/struct/]路径下的*_dm.toml文件");
    info!("init_2.2.2 {}", "解析[init/struct/]路径下的*_dm.toml文件");
    info!("init_2.2.3 {}", "缓存[init/struct/]路径下的数据模型");
    Ok("1.初始化平台表结构....".to_string())
}

///  init_db_bmbp_struct 初始化平台表结构
#[allow(dead_code)]
pub fn init_db_bmbp_struct_from_db() -> Result<String, String> {
    info!("init_3.3.{}", "从数据库中-初始化平台表结构....");
    Ok("1.初始化平台表结构....".to_string())
}

///  init_db_bmbp_api_from_rust 初始化平台接口
#[allow(dead_code)]
pub fn init_db_bmbp_api_from_rust() -> Result<String, String> {
    info!("init_4.{}", "从代码中-初始化平台表结构....");
    Ok("1.初始化平台表结构....".to_string())
}

///  init_db_bmbp_api_from_file 初始化平台接口
#[allow(dead_code)]
pub fn init_db_bmbp_api_from_file() -> Result<String, String> {
    info!("init_5.{}", "从文件中-初始化平台表结构....");
    Ok("1.初始化平台表结构....".to_string())
}

///  init_db_bmbp_api_from_db 初始化平台接口
#[allow(dead_code)]
pub fn init_db_bmbp_api_from_db() -> Result<String, String> {
    info!("init_6.{}", "从数据库中-初始化平台表结构....");
    Ok("1.初始化平台表结构....".to_string())
}

///  init_db_bus_data 初始化业务表数据
#[allow(dead_code)]
pub fn init_db_bus_data() -> Result<String, String> {
    info!("7.{}", "初始化业务表数据....");
    Ok("4.初始化业务表数据....".to_string())
}

pub fn load_bmbp_api_from_rust() {
    info!("init_3.1{}", "从代码中-初始化平台API...");
}

pub fn load_bmbp_api_from_file() {
    info!("init_3.2{}", "从文件中-初始化平台API...");
}
