use bmbp_abc::{base_ctx, BMBP_TERA};
use salvo::prelude::*;

/// 登录界面
#[handler]
pub async fn login_view(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    let mut ctx = base_ctx();
    let view_html = "login.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}
/// 登录逻辑
#[handler]
pub async fn do_login(req: &mut Request, resp: &mut Response, depot: &mut Depot) {}
/// 登录回调，用于sso登录
#[handler]
pub async fn login_call_back(req: &mut Request, resp: &mut Response, depot: &mut Depot) {}
