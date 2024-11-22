use crate::init::{init_app_router, init_template};
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
        tracing::info!("start app....");
        tracing::info!("init app web template ...... ");
        init_template();
        tracing::info!("init app router ......");
        let app_router = init_app_router();
        let acceptor = TcpListener::new("127.0.0.1:36000").bind().await;
        Server::new(acceptor).serve(app_router).await;
    }
}
