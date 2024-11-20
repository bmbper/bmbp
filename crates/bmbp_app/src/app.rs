use salvo::conn::TcpListener;
use salvo::routing::get;
use salvo::{handler, Listener, Router, Server};

#[handler]
async fn root() -> &'static str {
    "Hello, World!"
}
pub struct BmbpApp {}
impl BmbpApp {
    pub fn new() -> Self {
        BmbpApp {}
    }
    pub fn init(&mut self) {}
    pub async fn run(&mut self) {
        self.init();
        tracing_subscriber::fmt::init();
        tracing::info!("starting up");

        let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
        Server::new(acceptor).serve(router).await;
    }
}
