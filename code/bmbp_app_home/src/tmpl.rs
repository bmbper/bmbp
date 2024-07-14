use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "page"]
struct Asset;

lazy_static! {
    pub static ref home_tera: Tera = {
        let mut tera = Tera::default();
        // 加载嵌入的模板文件
        for file in Asset::iter() {
            if let Some(content) = Asset::get(file.as_ref()) {
                let content_str = std::str::from_utf8(content.data.as_ref()).expect("Failed to read template");
                tera.add_raw_template(file.as_ref(), content_str).expect("Failed to add template");
            }
        }
        tera
    };
}
