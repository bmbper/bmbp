use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::Level;

use bmbp_vars::map::{global_hash_map_vars, global_hash_map_vars_to_usize};
use env::init_app_env;
use routes::init_webapp_router;

mod env;
mod routes;

pub struct BmbpWebApp {
    router: Option<Router>,
}

impl BmbpWebApp {
    pub fn new() -> Self {
        BmbpWebApp { router: None }
    }

    pub fn init(&mut self) {
        // TRACE-DEBUG-INFO-WARN-ERROR
        tracing_subscriber::fmt()
            .with_level(true)
            .with_max_level(Level::INFO)
            .init();
        tracing::info!("初始化WebApp运行环境......");
        // 初始化环境变量
        init_app_env();
        self.init_router();
    }

    fn init_router(&mut self) {
        tracing::info!("初始化WebApp接口服务路由......");
        let mut router = Router::new();
        router = init_webapp_router(router);

        // 增加中间处理器
        router = router.layer(TraceLayer::new_for_http());
        self.router = Some(router);
    }

    pub async fn start(&mut self) {
        let host = self.host().clone();
        tracing::info!("启动WebApp应用服务,监听地址:{}......", host.clone());
        let addr = &host.parse().unwrap();
        let serve = self.router.as_mut().unwrap().clone().into_make_service();
        axum::Server::bind(addr).serve(serve).await.unwrap();
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
}
