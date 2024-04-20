use salvo::prelude::*;

use bmbp_app_common::map::{global_hash_map_vars, global_hash_map_vars_to_usize};
use env::init_app_env;
use routes::init_webapp_router;

mod data;
mod env;
mod routes;

pub struct BmbpWebApp {}

impl BmbpWebApp {
    pub fn new() -> Self {
        BmbpWebApp {}
    }

    pub fn init(&mut self) {
        tracing_subscriber::fmt().init();
        tracing::info!("初始化WebApp运行环境......");
        // 初始化环境变量
        init_app_env();
    }

    fn init_router(&mut self) -> Service {
        tracing::info!("初始化WebApp接口服务路由......");
        init_webapp_router()
    }

    pub async fn start(&mut self) {
        let host = self.host().clone();
        let context = self.app_context();
        tracing::info!(
            "启动WebApp应用服务,监听地址:{}{}......",
            host.clone(),
            context.clone()
        );
        let acceptor = TcpListener::new(host.as_str()).bind().await;
        let router = self.init_router();
        Server::new(acceptor).serve(router).await;
    }

    fn host(&self) -> String {
        let mut host = global_hash_map_vars("bmbp_server".to_string(), "server_host".to_string());
        if host.is_empty() {
            host = "0.0.0.0".to_string();
        }
        let mut port =
            global_hash_map_vars_to_usize("bmbp_server".to_string(), "server_port".to_string());
        if port == 0 {
            port = 3000;
        }
        format!("{}:{}", host, port)
    }
    fn app_context(&self) -> String {
        let mut context =
            global_hash_map_vars("bmbp_server".to_string(), "server_context".to_string());
        if context.is_empty() {
            context = "".to_string();
        }
        context
    }
}
