use crate::app::model::types::RbacAppType;
use ::serde::Deserialize;
use ::serde::Serialize;
use bmbp_app_common::*;
use bmbp_rdbc_marco::rdbc_model;
use bmbp_rdbc_orm::*;
use chrono::Utc;
use salvo::*;
use tracing::*;
#[rdbc_model(table = BMBP_RBAC_APP)]
pub struct BmbpRbacApp {
    // 应用编码
    app_code: Option<String>,
    // 应用名称
    #[query(like)]
    #[valid[require("应用名称不能为空"),unique("应用名称不能不能重复"),max_length(50,"应用名称不能超过50个字符"),min_length(2,"应用名称不能少于2个字符")]]
    app_name: Option<String>,
    // 应用简称
    #[query(like)]
    app_short_name: Option<String>,
    // 应用图标
    app_icon: Option<String>,
    // 应用描述
    app_desc: Option<String>,
    // 应用类型
    #[query(eq)]
    app_type: Option<RbacAppType>,
    // 应用主题
    app_theme: Option<String>,
    // 应用地址
    app_url: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::model::app_menu::BmbpRbacAppMenu;
    #[test]
    fn test_derive_model() {
        use super::*;
        let mut model = BmbpRbacApp::default();
        model.set_app_type(Some(RbacAppType::META));
        model.set_app_name(Some("test".to_string()));
        println!("{}", serde_json::to_string_pretty(&model).unwrap());
        let v = r#"{
                          "appCode": null,
                          "appName": "test",
                          "appShortName": null,
                          "appIcon": null,
                          "appDesc": null,
                          "appType": "SSO",
                          "appTheme": null,
                          "appUrl": null,
                          "dataId": null,
                          "dataLevel": null,
                          "dataFlag": null,
                          "dataStatus": null,
                          "dataSort": null,
                          "dataCreateTime": null,
                          "dataCreateUser": null,
                          "dataUpdateTime": null,
                          "dataUpdateUser": null,
                          "dataOwnerOrg": null,
                          "dataSign": null
                        }"#;
        if let Ok(m) = serde_json::from_str::<BmbpRbacApp>(&v) {
            println!("===")
        } else {
            println!("===ERROR====")
        }
    }
}
