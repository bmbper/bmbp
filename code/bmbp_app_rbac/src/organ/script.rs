use bmbp_app_common::BmbpBaseModel;
use bmbp_orm_ins::BmbpScriptSql;

pub(crate) struct OrganScript();

impl OrganScript {
    /// 获取组织机构信息表
    fn get_organ_table_name() -> String {
        "bmbp_rbac_organ".to_string()
    }
    fn get_organ_table_columns() -> Vec<String> {
        let base_field = BmbpBaseModel::get_fields();
        let organ_fields = Self::get_organ_fields();
        let mut fields = vec![];
        fields.extend_from_slice(base_field.as_slice());
        fields.extend_from_slice(organ_fields.as_slice());
        fields
    }
    fn get_organ_fields() -> Vec<String> {
        vec![
            "organ_code".to_string(),
            "organ_parent_code".to_string(),
            "organ_title".to_string(),
            "organ_code_path".to_string(),
            "organ_title_path".to_string(),
            "organ_data_id".to_string(),
            "organ_type".to_string(),
        ]
    }
}

impl OrganScript {
    /// 组织机构查询语句
    pub(crate) fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_organ_table_name().as_str());
        let columns = Self::get_organ_table_columns();
        query_script.select_string_slice(columns.as_slice());
        query_script
    }
}
