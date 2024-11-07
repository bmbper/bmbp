use salvo::conn::TcpListener;
use salvo::routing::get;
use salvo::{handler, Listener, Router, Server};

#[handler]
async fn root() -> &'static str {
    "Hello, World!"
}
pub struct BmbpApp {
    router: Router,
}
impl BmbpApp {
    pub fn new() -> Self {
        BmbpApp {
            router: Router::new(),
        }
    }
    pub fn init(&mut self) {}
    pub async fn run(&mut self) {
        self.init();
        tracing_subscriber::fmt::init();
        tracing::info!("starting up");
        let mut router = Router::new();
        let root_router = Router::with_path("").get(root);
        router = router.push(root_router);
        let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
        Server::new(acceptor).serve(router).await;
    }
}
