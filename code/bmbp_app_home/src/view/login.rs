use crate::inner::home_tera;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn login_view(req: &mut Request, res: &mut Response) {
    let mut ctx = tera::Context::new();
    res.render(Text::Html(home_tera.render("login.bpg", &ctx).unwrap()));
}
