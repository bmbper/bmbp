use bmbp_app_common::BMBP_TEMPLATE;
use salvo::handler;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
use tera::Context;

#[handler]
pub async fn login_view(_req: &mut Request, res: &mut Response) {
    let ctx = Context::new();
    let te = BMBP_TEMPLATE.render("base/login/login.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}
