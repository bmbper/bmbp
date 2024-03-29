use bmbp_app::BmbpWebApp;
use tokio;

#[tokio::main]
async fn main() {
    let mut bmbp_web_app = BmbpWebApp::new();
    bmbp_web_app.init();
    bmbp_web_app.start().await
}
