use crate::init::{init_app_router, init_template, init_white_list, initialize_orm};
use bmbp_vars::app_locale;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};

pub struct BmbpApp {}
impl BmbpApp {
    pub fn new() -> Self {
        BmbpApp {}
    }
    pub async fn init(&self) {
        tracing_subscriber::fmt::init();
        tracing::info!("start_app");
        tracing::info!("init app web template ...... ");
        tracing::info!("init white list:{}", app_locale());
        init_white_list();
        init_template();
        initialize_orm().await
    }
    pub async fn run(&mut self) {
        self.init().await;
        tracing::info!("init app router ......");
        let app_router = init_app_router();
        let acceptor = TcpListener::new("127.0.0.1:36000").bind().await;
        Server::new(acceptor).serve(app_router).await;
    }
}
