use axum::routing::get;
use axum::Router;

pub struct BmbpApp;
impl BmbpApp {
    pub fn new() -> Self {
        BmbpApp
    }
    pub async fn run(&self) {
        tracing_subscriber::fmt::init();
        tracing::info!("starting up");
        async fn root() -> &'static str {
            "Hello, World!"
        }
        // build our application with a route
        let app = Router::new().route("/", get(root));

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
