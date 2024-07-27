use bmbp_app::BmbpWebApp;
use tokio;

#[tokio::main]
async fn main() {
    BmbpWebApp::new().start().await
}
