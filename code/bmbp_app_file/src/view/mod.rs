use crate::tmpl::file_tera;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn file_view(req: &mut Request, res: &mut Response) {
    let mut ctx = tera::Context::new();
    res.render(Text::Html(file_tera.render("file.bpg", &ctx).unwrap()));
}
