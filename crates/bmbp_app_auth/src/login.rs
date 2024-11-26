use crate::bean::LoginUser;
use crate::middle::get_token_value;
use bmbp_abc::{base_ctx, BMBP_TERA};
use bmbp_auth::{BmbpAuthUtil, BmbpToken};
use bmbp_bean::RespVo;
use http::header::SET_COOKIE;
use salvo::http::cookie::Cookie;
use salvo::prelude::*;

/// 登录界面
#[handler]
pub async fn login_view(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    let ctx = base_ctx();
    let view_html = "login.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}
/// 登录逻辑
#[handler]
pub async fn do_login(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    let login_user = req.parse_json::<LoginUser>().await.unwrap();

    //TODO 校验用户名和密码
    if (login_user.username == "admin" && login_user.password == "bced6fd149cfcdb85741768da12e41c6")
        || (login_user.username == "bmbp"
            && login_user.password == "063a891eda0dc73ee3352934ebe6257b")
    {
        let token = BmbpAuthUtil::login_by_uid(login_user.username.clone());
        let resp_vo = RespVo::<BmbpToken> {
            code: 0,
            msg: "登录成功".to_string(),
            data: Some(token.clone()),
        };
        let cookie = Cookie::parse(format!(
            "token={}; Path=/; HttpOnly",
            token.token.as_ref().unwrap().clone()
        ))
        .unwrap();
        resp.headers_mut()
            .append(SET_COOKIE, cookie.to_string().parse().unwrap());
        resp.render(Json(resp_vo))
    } else {
        let resp_vo = RespVo::<String> {
            code: -1,
            msg: "用户名或密码错误".to_string(),
            data: None,
        };
        resp.render(Json(resp_vo))
    }
}
/// 登录回调，用于sso登录
#[handler]
pub async fn login_call_back(req: &mut Request, resp: &mut Response, depot: &mut Depot) {}

#[handler]
pub async fn login_out(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Json<RespVo<String>> {
    let token = get_token_value(req);
    BmbpAuthUtil::logout_by_token(token);
    let resp_vo = RespVo::<String> {
        code: 0,
        msg: "退出成功".to_string(),
        data: None,
    };
    Json(resp_vo)
}
