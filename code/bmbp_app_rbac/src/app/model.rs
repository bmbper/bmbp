use bmbp_app_common::BmbpBaseModel;

/// 应用信息
#[allow(dead_code)]
pub struct BmbpRbacApp {
    base: BmbpBaseModel,
    app_code: Option<String>,
    app_title: Option<String>,
    app_key: Option<String>,
    app_secret_key: Option<String>,
    app_type: Option<BmbpRbacAppType>,
}

impl BmbpRbacApp {
    pub fn orm_table_name() -> String {
        "bmbp_rbac_app".to_string()
    }

    pub fn orm_table_column_name() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_fields();
        let rbac_app_field = vec![
            "app_code".to_string(),
            "app_title".to_string(),
            "app_key".to_string(),
            "app_secret_key".to_string(),
            "app_type".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}

/// 应用类型
#[allow(dead_code)]
pub enum BmbpRbacAppType {
    /// 模块应用
    MODULE,
    ///单点应用
    SSO,
    /// 连接应用
    LINK,
}
