mod bean;
pub use bean::*;

use lazy_static::lazy_static;
use std::fs;
use toml::de::Error;
lazy_static! {
    pub static ref BmbpVar: BmbpVars = {
        let mut bmbp_vars = BmbpVars::default();
        let config_path = "config/bmbp.toml";
        if let Ok(config_rs) = fs::read_to_string(config_path) {
            let bmbp_vars_rs: Result<BmbpVars, Error> = toml::from_str(config_rs.as_str());
            match bmbp_vars_rs {
                Ok(vars) => {
                    bmbp_vars = vars;
                }
                Err(er) => {
                    tracing::warn!(
                        "配置文件config/bmbp.toml读取失败:{}，使用默认配置",
                        er.to_string()
                    );
                }
            }
        } else {
            tracing::warn!("配置文件config/bmbp.toml不存在，使用默认配置");
        }
        bmbp_vars
    };
}

#[cfg(test)]
mod tests {
    use crate::{BmbpVar, BmbpVars};
    use std::fs;
    use toml::de::Error;
    #[test]
    fn test_vars() {
        tracing_subscriber::fmt().init();
        let config_path = "../../config/bmbp.toml";
        if let Ok(bmbp_config_rs) = fs::read_to_string(config_path) {
            let bmbp_vars_op: Result<BmbpVars, Error> = toml::from_str(bmbp_config_rs.as_str());
            match bmbp_vars_op {
                Ok(bmbp) => {
                    tracing::info!("读取配置{:#?}", bmbp);
                }
                Err(er) => {
                    tracing::warn!(
                        "配置文件config/bmbp.toml读取失败:{}，使用默认配置",
                        er.to_string()
                    );
                }
            }
        } else {
            tracing::warn!("配置文件config/bmbp.toml不存在，使用默认配置");
        }
    }
    /// 内部测试时，配置文件路径与生产时路径不一样
    #[test]
    fn test_gloabl_vars() {
        tracing_subscriber::fmt().init();
        match BmbpVar.bmbp.as_ref() {
            Some(bmbp) => match bmbp.app.as_ref() {
                Some(app) => match app.code.as_ref() {
                    Some(code) => {
                        assert_eq!(code, "bmbp");
                    }
                    None => {}
                },
                None => {}
            },
            None => {}
        }
    }
}
