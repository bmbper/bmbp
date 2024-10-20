use crate::inner::home_tera;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn login_view(_req: &mut Request, res: &mut Response) {
    let ctx = tera::Context::new();
    res.render(Text::Html(home_tera.render("login.btl", &ctx).unwrap()));
}
