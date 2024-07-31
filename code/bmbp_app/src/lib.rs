use bmbp_app_init::build_init_app_router;
use bmbp_app_vars::BmbpVar;
use salvo::prelude::*;

use routes::init_webapp_router;

mod data;
mod env;
mod routes;

pub struct BmbpWebApp {
    ready: bool,
}

impl BmbpWebApp {
    pub fn new() -> Self {
        BmbpWebApp { ready: false }
    }

    pub async fn start(&mut self) {
        tracing_subscriber::fmt().init();
        tracing::info!("初始化应用运行环境......");
        self.ready = BmbpVar.valid();
        if self.ready {
            self.start_web_app().await
        } else {
            self.start_init_app().await
        }
    }

    async fn start_init_app(&self) {
        let host = "0.0.0.0:7027";
        tracing::info!("启动初始化服务,监听地址:{}......", host);
        let acceptor = TcpListener::new(host).bind().await;
        let router = build_init_app_router();
        Server::new(acceptor).serve(router).await;
    }

    async fn start_web_app(&self) {
        let host = self.host().clone();
        tracing::info!("启动应用服务,监听地址:{}......", host.clone());
        let acceptor = TcpListener::new(host.as_str()).bind().await;
        let router = init_webapp_router();
        Server::new(acceptor).serve(router).await;
    }

    fn host(&self) -> String {
        let mut server_host = "0.0.0.0".to_string();
        let mut server_port = 7027u32;
        if let Some(bmbp) = BmbpVar.bmbp.as_ref() {
            if let Some(server) = bmbp.server.as_ref() {
                if let Some(host) = server.host.as_ref() {
                    if !host.is_empty() {
                        server_host = host.to_string();
                    }
                }
                if let Some(port) = server.port.as_ref() {
                    if port != &0u32 {
                        server_port = port.clone();
                    }
                }
            }
        }
        format!("{}:{}", server_host, server_port)
    }
}
