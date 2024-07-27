use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BmbpVars {
    pub bmbp: Option<Bmbp>,
}
impl BmbpVars {
    pub fn valid(&self) -> bool {
        if self.bmbp.is_none() {
            tracing::warn!("配置文件缺少bmbp,进入配置系统");
            return false;
        }
        if self.bmbp.as_ref().unwrap().app.is_none() {
            tracing::warn!("配置文件缺少app,进入配置系统");
            return false;
        }
        if self.bmbp.as_ref().unwrap().server.is_none() {
            tracing::warn!("配置文件缺少server,进入配置系统");

            return false;
        }
        if self.bmbp.as_ref().unwrap().datasource.is_none() {
            tracing::warn!("配置文件缺少datasource,进入配置系统");
            return false;
        }
        let ds = self.bmbp.as_ref().unwrap().datasource.as_ref().unwrap();
        if ds.driver.is_none() || ds.username.is_none() {
            tracing::warn!("配置文件datasource缺少drvier或username,进入配置系统");
            return false;
        }

        return true;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bmbp {
    pub app: Option<App>,
    pub server: Option<Server>,
    pub datasource: Option<DataSource>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    pub code: Option<String>,
    pub name: Option<String>,
    pub login_name: Option<String>,
    pub nav_name: Option<String>,
    pub copy_right: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Server {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub env: Option<String>,
    pub log_level: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataSource {
    pub driver: Option<String>,
    pub host: Option<String>,
    pub port: Option<u32>,
    pub database: Option<String>,
    pub schema: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ignore_case: Option<bool>,
    pub init: Option<u32>,
    pub max_idle: Option<u32>,
    pub min_idle: Option<u32>,
    pub max_time_wait: Option<u32>,
    pub max_connect_time_wait: Option<u32>,
}
