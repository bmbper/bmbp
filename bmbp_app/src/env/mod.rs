use crate::env::vars::load_config_to_global_vars;

mod vars;

pub fn init_app_env() {
    tracing::info!("初始始平台环境变量......");
    load_config_to_global_vars();
}
