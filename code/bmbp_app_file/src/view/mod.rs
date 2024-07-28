use crate::tmpl::file_tera;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn file_view(_req: &mut Request, res: &mut Response) {
    let ctx = tera::Context::new();
    res.render(Text::Html(file_tera.render("file.btl", &ctx).unwrap()));
}
