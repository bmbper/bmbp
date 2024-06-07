use bmbp_app_common::{BmbpResp, RespVo};
use salvo::{handler, Request, Response};
use serde_json::Value;

#[handler]
pub async fn query_view_config(req: &mut Request, _res: &mut Response) -> BmbpResp<RespVo<Value>> {
    let view_code_rs = req.param::<String>("viewCode");
    match view_code_rs {
        Some(view_code) => {
            return Ok(RespVo::ok_data(Value::String(format!(
                "{}获取视图信息",
                view_code
            ))));
        }
        None => Ok(RespVo::fail_msg("请提定视图编码[viewCode]")),
    }
}
