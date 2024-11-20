use crate::routes::build_app_router;
use salvo::conn::TcpListener;
use salvo::routing::get;
use salvo::{handler, Listener, Router, Server};

pub struct BmbpApp {}
impl BmbpApp {
    pub fn new() -> Self {
        BmbpApp {}
    }
    pub async fn run(&mut self) {
        tracing_subscriber::fmt::init();
        tracing::info!("启动应用程序....");
        let app_router = build_app_router();
        let acceptor = TcpListener::new("127.0.0.1:36000").bind().await;
        Server::new(acceptor).serve(app_router).await;
    }
}
