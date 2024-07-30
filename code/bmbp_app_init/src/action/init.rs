use bmbp_app_common::HttpRespVo;
use bmbp_app_common::RespVo;
use bmbp_app_vars::BmbpVars;
use salvo::handler;
use salvo::prelude::*;
#[handler]
pub async fn save_config(req: &mut Request, res: &mut Response) -> HttpRespVo<String> {
    let vars = req.parse_json::<BmbpVars>().await?;
    let (valid, msg) = vars.valid_msg();
    if valid {
        Ok(RespVo::ok_data("新的服务IP".to_string()))
    } else {
        Ok(RespVo::fail_msg(msg.as_str()))
    }
}
