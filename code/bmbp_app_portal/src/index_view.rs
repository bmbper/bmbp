use bmbp_app_common::EmbedStatic;
use bmbp_app_common::APP_TITLE;
use bmbp_app_common::BMBP_TEMPLATE;
use salvo::handler;
use salvo::serve_static::EmbeddedFileExt;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
use tera::Context;

#[handler]
pub async fn index_view(_req: &mut Request, res: &mut Response) {
    let mut ctx = Context::new();
    ctx.insert("appTitle", APP_TITLE);
    ctx.insert("viewScript", vec!["portal/index.js"].as_slice());
    ctx.insert("viewCss", vec!["portal/index.css"].as_slice());
    ctx.insert("viewName", "IndexView");
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}

#[handler]
pub async fn favicon(req: &mut Request, res: &mut Response) {
    if let Some(file) = EmbedStatic::get("asserts/image/favicon.ico") {
        file.render(req, res);
    }
}
