use crate::types::http::BmbpHttp;

/// http_err
pub fn err_http(msg: String) -> BmbpHttp<String> {
    BmbpHttp {
        code: -1,
        msg,
        data: None,
    }
}

/// http_ok
pub fn ok_http<T>(msg: String, data: Option<T>) -> BmbpHttp<T> {
    BmbpHttp { code: 1, msg, data }
}
