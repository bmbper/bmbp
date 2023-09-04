use bmbp_app_common::page_context;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BMBP_TEMPLATE;
use bmbp_app_utils::render_to_json_string;
use salvo::handler;
use salvo::writing::Text;
use salvo::Request;
use salvo::Response;
#[handler]
pub async fn app_index_view(_req: &mut Request, res: &mut Response) {
    let ctx = page_context("rbac/app/index");
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}

#[handler]
pub async fn app_config_view(req: &mut Request, res: &mut Response) {
    let mut ctx = page_context("rbac/app/config");
    let page_params = req
        .parse_queries::<BmbpHashMap>()
        .unwrap_or(BmbpHashMap::new());
    let page_prams_json_str = &render_to_json_string(&page_params);
    ctx.insert("PageVars", page_prams_json_str);
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}

#[handler]
pub async fn app_base_view(_req: &mut Request, res: &mut Response) {
    let ctx = page_context("rbac/app/base");
    let te = BMBP_TEMPLATE.render("page.html", &ctx).unwrap();
    res.render(Text::Html(&te))
}
