use crate::init::init_tera;
use bmbp_app_vars::BmbpVar;
use salvo::handler;
use salvo::prelude::*;

#[handler]
pub async fn root_view(_: &mut Request, resp: &mut Response) {
    resp.render(Redirect::found("/init/index.view"))
}

#[handler]
pub async fn index_view(_: &mut Request, res: &mut Response) {
    let config_vars = BmbpVar.clone();
    let config_vars_string = serde_json::to_string(&config_vars).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("bmbp", &config_vars_string);
    res.render(Text::Html(init_tera.render("init.btl", &ctx).unwrap()));
}
