use bmbp_app_common::BmbpBaseModel;
use bmbp_orm_ins::BmbpScriptSql;

/// RbacAppScript  应用的SQL脚本
pub struct RbacAppScript;

impl RbacAppScript {
    pub fn get_table_name() -> String {
        "bmbp_rbac_app".to_string()
    }
    pub fn get_table_columns() -> Vec<String> {
        let mut base_fields = BmbpBaseModel::get_fields();
        let rbac_app_field = vec![
            "app_code".to_string(),
            "app_title".to_string(),
            "app_key".to_string(),
            "app_secrect_key".to_string(),
            "app_type".to_string(),
        ];
        base_fields.extend_from_slice(rbac_app_field.as_slice());
        base_fields
    }
}

impl RbacAppScript {
    pub fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_table_name().as_str());
        let columns = Self::get_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
}
