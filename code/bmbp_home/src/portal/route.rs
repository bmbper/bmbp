use crate::portal::model::NavAppMenuVo;
use axum::response::IntoResponse;
use bmbp_types::{BmbpError, BmbpResp, RespVo};
use serde_json::Error;

pub async fn find_app() -> BmbpResp<impl IntoResponse> {
    Ok("".to_string())
}

pub async fn find_app_menu() -> BmbpResp<impl IntoResponse> {
    let menu = r#"
    [
		{
			"title": "系统管理",
			"route": "11111111111111111111",
			"children": [
				{
					"title": "组织管理",
					"route": "portal/sys/rbac/organ"
				},
				{
					"title": "用户管理",
					"route": "portal/sys/rbac/user"
				},
				{
					"title": "菜单管理",
					"route": "portal/sys/rbac/menu"
				},
				{
					"title": "角色管理",
					"route": "portal/sys/rbac/role"
				},
				{
					"title": "接口管理",
					"route": "portal/sys/rbac/api"
				},
				{
					"title": "数据管理",
					"route": "portal/sys/rbac/data"
				}
			]
		},
		{
			"title": "系统设置",
			"route": "",
			"children": [
				{
					"title": "行政区划",
					"route": ""
				},
				{
					"title": "数据字典",
					"route": ""
				},
				{
					"title": "运行参数",
					"route": ""
				},
				{
					"title": "安全配置",
					"route": ""
				}
			]
		},
		{
			"title": "平台审计",
			"route": "",
			"children": [
				{
					"title": "日志审计",
					"route": ""
				},
				{
					"title": "资源统计",
					"route": ""
				},
				{
					"title": "缓存查询",
					"route": ""
				}
			]
		}
	]
  "#;
    let vo_rs: Result<Vec<NavAppMenuVo>, Error> = serde_json::from_str(menu);
    match vo_rs {
        Ok(v) => Ok(RespVo::ok_data(v)),
        Err(err) => Err(BmbpError::api(err.to_string())),
    }
}
