use crate::tmpl::home_tera;
use salvo::writing::Text;
use salvo::{handler, Request, Response, Scribe};
#[handler]
pub async fn home_view(req: &mut Request, res: &mut Response) {
    let mut ctx = tera::Context::new();
    res.render(Text::Html(home_tera.render("home.bpg", &ctx).unwrap()));
}
