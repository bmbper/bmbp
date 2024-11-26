use bmbp_vars::{
    app_copy_right, app_email, app_group_name, app_home_url, app_icon, app_login_name, app_name,
    app_short_name, app_title, app_version, BMBP_APP_COPY_WRITE, BMBP_APP_EMAIL,
    BMBP_APP_GROUP_NAME, BMBP_APP_HOME_URL, BMBP_APP_ICON, BMBP_APP_LOGIN_NAME, BMBP_APP_NAME,
    BMBP_APP_SHORT_NAME, BMBP_APP_TITLE, BMBP_APP_VERSION,
};
use rust_embed::EmbeddedFile;
use std::sync::{LazyLock, RwLock};
use tera::{Context, Tera};

pub fn base_ctx() -> Context {
    let mut ctx = Context::new();
    ctx.insert(BMBP_APP_TITLE, &app_title());
    ctx.insert(BMBP_APP_ICON, &app_icon());
    ctx.insert(BMBP_APP_NAME, &app_name());
    ctx.insert(BMBP_APP_SHORT_NAME, &app_short_name());
    ctx.insert(BMBP_APP_GROUP_NAME, &app_group_name());
    ctx.insert(BMBP_APP_LOGIN_NAME, &app_login_name());
    ctx.insert(BMBP_APP_EMAIL, &app_email());
    ctx.insert(BMBP_APP_VERSION, &app_version());
    ctx.insert(BMBP_APP_COPY_WRITE, &app_copy_right());
    ctx.insert(BMBP_APP_HOME_URL, &app_home_url());
    ctx
}

pub static BMBP_TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    RwLock::new(tera)
});

pub fn tera_add_template(file_path: &str, file: Option<EmbeddedFile>) {
    if let Some(content) = file {
        let content_str =
            std::str::from_utf8(content.data.as_ref()).expect("loading plugin template fail ....");
        BMBP_TERA
            .write()
            .unwrap()
            .add_raw_template(file_path, content_str)
            .expect("add plugin template to tera engin fail....");
    }
}
