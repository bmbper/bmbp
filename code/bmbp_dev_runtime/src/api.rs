use bmbp_app_common::{BmbpResp, RespVo};
use salvo::{handler, Request, Response};
use serde_json::Value;

#[handler]
pub async fn execute_api(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<Value>> {
    let api_code_rs = req.param::<String>("apiCode");
    match api_code_rs {
        Some(api_code) => {
            return Ok(RespVo::ok_data(Value::String(format!(
                "{}执行接口",
                api_code
            ))));
        }
        None => Ok(RespVo::fail_msg("请提定接口编码[apiId]")),
    }
}
