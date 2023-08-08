use bmbp_app_common::{BmbpBaseModel, BmbpHashMap, BmbpValue, PageParams};
use serde::{Deserialize, Serialize};

/// AppQueryParams 应用查询类型
pub type AppQueryParams = PageParams<BmbpRbacApp>;

/// BmbpRbacApp 应用信息
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacApp {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 应用编码
    app_code: Option<String>,
    /// 应用名称
    app_title: Option<String>,
    /// 应用标识
    app_key: Option<String>,
    /// 应用密钥
    app_secret_key: Option<String>,
    /// 应用类型
    app_type: Option<String>,
}

impl From<&BmbpRbacApp> for BmbpValue {
    fn from(value: &BmbpRbacApp) -> Self {
        let hash_map = BmbpHashMap::from(value);
        BmbpValue::Map(hash_map)
    }
}

impl From<&BmbpRbacApp> for BmbpHashMap {
    fn from(value: &BmbpRbacApp) -> Self {
        let mut hash_map = BmbpHashMap::new();
        hash_map.insert("base".to_string(), BmbpValue::from(value.get_base_model()));
        hash_map.insert(
            "app_code".to_string(),
            BmbpValue::from(value.get_app_code()),
        );
        hash_map.insert(
            "app_title".to_string(),
            BmbpValue::from(value.get_app_title()),
        );
        hash_map.insert("app_key".to_string(), BmbpValue::from(value.get_app_key()));
        hash_map.insert(
            "app_secret_key".to_string(),
            BmbpValue::from(value.get_app_secret_key()),
        );
        hash_map.insert(
            "app_type".to_string(),
            BmbpValue::from(value.get_app_type()),
        );
        hash_map
    }
}

#[allow(dead_code)]
impl BmbpRbacApp {
    pub fn new() -> Self {
        BmbpRbacApp::default()
    }

    pub fn set_base_model(&mut self, model: BmbpBaseModel) -> &mut Self {
        self.base = model;
        self
    }

    pub fn set_app_code(&mut self, app_code: String) -> &mut Self {
        self.app_code = Some(app_code);
        self
    }
    pub fn set_app_title(&mut self, app_title: String) -> &mut Self {
        self.app_title = Some(app_title);
        self
    }
    pub fn set_app_key(&mut self, app_key: String) -> &mut Self {
        self.app_key = Some(app_key);
        self
    }
    pub fn set_app_secret_key(&mut self, app_secret_key: String) -> &mut Self {
        self.app_secret_key = Some(app_secret_key);
        self
    }
    pub fn set_app_type(&mut self, app_type: BmbpRbacAppType) -> &mut Self {
        self.app_type = Some(app_type.to_string());
        self
    }

    pub fn get_mut_base_model(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }

    pub fn get_base_model(&self) -> &BmbpBaseModel {
        &self.base
    }

    pub fn get_app_code(&self) -> Option<&String> {
        self.app_code.as_ref()
    }

    pub fn get_app_title(&self) -> Option<&String> {
        self.app_title.as_ref()
    }
    pub fn get_app_key(&self) -> Option<&String> {
        self.app_key.as_ref()
    }
    pub fn get_app_secret_key(&self) -> Option<&String> {
        self.app_secret_key.as_ref()
    }

    pub fn get_app_type(&self) -> Option<&String> {
        self.app_type.as_ref()
    }
}

/// 应用类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum BmbpRbacAppType {
    /// 内部模块应用
    MODULE,
    /// 单点集成应用
    SSO,
    /// 链接应用
    LINK,
}

impl Default for BmbpRbacAppType {
    fn default() -> Self {
        BmbpRbacAppType::MODULE
    }
}
impl ToString for BmbpRbacAppType {
    fn to_string(&self) -> String {
        match self {
            BmbpRbacAppType::MODULE => "MODULE".to_string(),
            BmbpRbacAppType::SSO => "SSO".to_string(),
            BmbpRbacAppType::LINK => "LINK".to_string(),
        }
    }
}
impl From<String> for BmbpRbacAppType {
    fn from(value: String) -> Self {
        if value.eq_ignore_ascii_case("MODULE") {
            return BmbpRbacAppType::MODULE;
        }

        if value.eq_ignore_ascii_case("SSO") {
            return BmbpRbacAppType::SSO;
        }

        if value.eq_ignore_ascii_case("LINK") {
            return BmbpRbacAppType::LINK;
        }

        return BmbpRbacAppType::MODULE;
    }
}

impl From<&BmbpRbacAppType> for BmbpValue {
    fn from(value: &BmbpRbacAppType) -> Self {
        BmbpValue::from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::model::BmbpRbacApp;

    #[test]
    pub fn test_serde_der() {
        let json = r#"
            [
              {
                "recordSign": "1",
                "recordLevel": "0",
                "appTitle": "1",
                "recordUpdateTime": "1",
                "appType": "sso",
                "recordId": "111",
                "recordCreateTime": "1",
                "appCode": "111",
                "recordOwnerUser": "1",
                "recordUpdateUser": "1",
                "recordStatus": "0",
                "recordRemark": "1",
                "appKey": "1",
                "appSecrectKey": "1",
                "recordNum": null,
                "recordOwnerOrg": "1",
                "recordFlag": "0",
                "recordCreateUser": "1"
              }
            ]

            "#;

        let d: Vec<BmbpRbacApp> = serde_json::from_str(json).unwrap();
        println!("{:?}", d);
    }
}
