use bmbp_app_common::APP_TITLE;
use bmbp_app_common::BMBP_TEMPLATE;
use salvo::handler;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
use tera::Context;
#[handler]
pub async fn app_index_view(_req: &mut Request, res: &mut Response) {
    let mut ctx = Context::new();
    ctx.insert("appTitle", APP_TITLE);
    ctx.insert(
        "viewScript",
        vec![
            "rbac/app/app.js",
            "rbac/app/app_edit.js",
            "rbac/app/app_config.js",
            "rbac/app/app_info.js",
            "rbac/app/api.js",
        ]
        .as_slice(),
    );
    ctx.insert("viewName", "RbacAppView");
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}
