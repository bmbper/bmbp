use bmbp_app::BmbpApp;
use rust_embed::RustEmbed;
use rust_i18n::t;
#[derive(RustEmbed)]
#[folder = "locales"]
pub struct I18nLocales;
rust_i18n::i18n!("locales", fallback = "zh_cn");
#[tokio::main]
async fn main() {
    rust_i18n::set_locale("zh_cn");
    println!("{}", t!("start_app"));
    BmbpApp::new().run().await;
}
