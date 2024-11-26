use bmbp_auth::BmbpAuthUtil;
use bmbp_vars::{app_token_name, app_white_list_url};
use http::header::SET_COOKIE;
use salvo::http::headers::SetCookie;
use salvo::prelude::*;
use wildmatch::WildMatch;

/// 认证中间件
#[handler]
pub async fn auth_middle(req: &mut Request, depot: &mut Depot, resp: &mut Response) {
    let req_path = req.uri().path();
    if match_white_list_url(req_path) {
        return;
    }
    let token_value = get_token_value(req);
    if token_value.is_empty() {
        resp.render(Redirect::found("/auth/login.view"));
        return;
    }
    match BmbpAuthUtil::get_session_by_token(token_value) {
        Some(session) => {
            depot.insert("session", session);
        }
        None => {
            resp.headers.remove(SET_COOKIE);
            resp.render(Redirect::found("/auth/login.view"));
            return;
        }
    }
}

pub fn match_white_list_url(path: &str) -> bool {
    let white_list = app_white_list_url();
    if white_list.is_empty() {
        return false;
    }
    let wild_match_list = white_list
        .iter()
        .map(|x| WildMatch::new(x))
        .collect::<Vec<_>>();
    wild_match_list.iter().any(|x| x.matches(path))
}

pub fn get_token_value(req: &Request) -> String {
    let app_token_name = app_token_name();
    if app_token_name.is_empty() {
        get_token_value_default(req)
    } else {
        get_token_value_by_name(app_token_name, req)
    }
}

fn get_token_value_by_name(token_name: String, req: &Request) -> String {
    /// header 中传值
    let token_value = req
        .headers()
        .get(token_name.clone())
        .map(|x| x.to_str().unwrap());
    if token_value.is_some() {
        return token_value.unwrap().to_string();
    };
    /// cookie 中传值
    let cookie_value = req.cookie(token_name.as_str());
    if cookie_value.is_some() {
        let cookie_value = cookie_value.unwrap();
        return cookie_value.value().to_string();
    };
    /// 参数中传值
    let token_value: Option<String> = req.query(token_name.as_str());
    if token_value.is_some() {
        return token_value.unwrap();
    }
    "".to_string()
}

fn get_token_value_default(req: &Request) -> String {
    let token_names = vec!["Authorization", "token", "access_token", "bmbp_token"];
    for token_name in token_names {
        let token_value = get_token_value_by_name(token_name.to_string(), req);
        if !token_value.is_empty() {
            return token_value;
        };
    }
    "".to_string()
}
