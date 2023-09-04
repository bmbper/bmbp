use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::APP_TITLE;
use bmbp_app_common::BMBP_TEMPLATE;
use bmbp_app_utils::render_to_json_string;
use salvo::handler;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
use tera::Context;
#[handler]
pub async fn menu_index_view(req: &mut Request, res: &mut Response) {
    let page_params = req
        .parse_queries::<BmbpHashMap>()
        .unwrap_or(BmbpHashMap::new());
    let mut ctx = Context::new();
    ctx.insert("appTitle", APP_TITLE);
    ctx.insert(
        "viewScript",
        vec![
            "rbac/menu/index.js",
            "rbac/menu/form.js",
            "rbac/menu/api.js",
        ]
        .as_slice(),
    );
    ctx.insert("viewName", "PageView");
    let page_prams_json_str = &render_to_json_string(&page_params);
    tracing::info!("{}", page_prams_json_str);
    ctx.insert("pageParams", page_prams_json_str);
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}
