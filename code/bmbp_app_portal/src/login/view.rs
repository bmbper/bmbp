use bmbp_app_common::APP_TITLE;
use bmbp_app_common::BMBP_TEMPLATE;
use salvo::handler;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
use tera::Context;

#[handler]
pub async fn login_view(_req: &mut Request, res: &mut Response) {
    let mut ctx = Context::new();
    ctx.insert("appTitle", APP_TITLE);
    ctx.insert("viewScript", vec!["login/login.js"].as_slice());
    ctx.insert("viewCss", vec!["login.css"].as_slice());
    ctx.insert("viewName", "LoginView");
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}
