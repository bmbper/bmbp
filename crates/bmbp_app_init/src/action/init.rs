use std::fs::File;
use std::io::Write;

#[handler]
pub async fn save_config(req: &mut Request, res: &mut Response) -> HttpRespVo<String> {
    let vars = req.parse_json::<BmbpVars>().await?;
    let (valid, msg) = vars.valid_msg();
    if valid {
        let host = vars.server_host();
        let config_toml = toml::to_string(&vars).unwrap();
        let config_path = "config/bmbp.toml";
        if let Ok(mut config_file) = File::create(config_path) {
            if let Ok(_) = config_file.write_all(config_toml.as_bytes()) {
                return Ok(RespVo::ok_data(host));
            } else {
                return Ok(RespVo::fail_msg("写入文件失败，请重试！"));
            }
        } else {
            Ok(RespVo::fail_msg("创建配置文件失败，请重试！"))
        }
    } else {
        Ok(RespVo::fail_msg(msg.as_str()))
    }
}

#[handler]
pub async fn valid_datasource(req: &mut Request, res: &mut Response) -> HttpRespVo<String> {
    let vars = req.parse_json::<DataSource>().await?;
    let (valid, msg) = vars.valid_msg();
    if valid {
        Ok(RespVo::ok_msg("数据源验证成功"))
    } else {
        Ok(RespVo::fail_msg(msg.as_str()))
    }
}
