use axum::routing::get;
use axum::Router;

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
    pub fn init(&mut self) {
        let mut router = Router::new();
        let root_router = Router::new().route("/", get(root));
        router = router.merge(root_router);
        self.router = router;
    }
    pub async fn run(&mut self) {
        self.init();
        tracing_subscriber::fmt::init();
        tracing::info!("starting up");

        // build our application with a route
        let app = Router::new().route("/", get(root));

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
