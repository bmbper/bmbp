use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;
use salvo::Router;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "page"]
pub struct PageAsset;

lazy_static! {
    pub static ref init_tera: Tera = {
        let mut tera = Tera::default();
        // 加载嵌入的模板文件
        for file in PageAsset::iter() {
            if let Some(content) = PageAsset::get(file.as_ref()) {
                let content_str = std::str::from_utf8(content.data.as_ref()).expect("Failed to read template");
                tera.add_raw_template(file.as_ref(), content_str).expect("Failed to add template");
            }
        }
        tera
    };
}

#[derive(RustEmbed)]
#[folder = "lib"]
pub struct ScriptAsset;

pub fn build_init_script_router() -> Router {
    Router::with_path("ui/init/<**path>").get(static_embed::<ScriptAsset>())
}
