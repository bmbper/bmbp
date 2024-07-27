use crate::init::init_tera;
use salvo::handler;
use salvo::prelude::*;
#[handler]
pub async fn init_view(_: &mut Request, res: &mut Response) {
    let ctx = tera::Context::new();
    res.render(Text::Html(init_tera.render("init.btl", &ctx).unwrap()));
}
