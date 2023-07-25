use bmbp_app_common::BMBP_TEMPLATE;
use salvo::handler;
use salvo::Request;
use salvo::Response;
use tera::Context;

#[handler]
pub async fn app_index_view(req: &mut Request, res: &mut Response) {
    let mut ctx = Context::new();
    ctx.insert("name", "zhangguokai");
    let te = BMBP_TEMPLATE.render("index.html", &ctx).unwrap();
    res.render(&te)
}
