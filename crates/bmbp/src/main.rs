use bmbp_app::BmbpApp;

#[tokio::main]
async fn main() {
    BmbpApp::new().run().await;
}
