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
    pub(crate) fn insert_script() -> BmbpScriptSql {
        let mut insert = BmbpScriptSql::new();
        insert.insert_into(Self::get_organ_table_name().as_str());
        for field in Self::get_organ_table_columns().as_slice() {
            insert.insert_value(field, format!("#{}", field).as_str());
        }
        insert
    }
    pub(crate) fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_organ_table_name().as_str());
        update
    }
    pub(crate) fn update_status_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("record_status = #{record_status}");
        script.filter("record_id = #{record_id}");
        script
    }
    pub(crate) fn update_parent_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("organ_parent_code = #{organ_parent_code}");
        script.filter("record_id = #{record_id}");
        script
    }
    pub(crate) fn delete_script_by_id() -> BmbpScriptSql {
        let mut script = Self::delete_script();
        script.filter("record_id in (#{record_id})");
        script
    }
    pub(crate) fn delete_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.delete(Self::get_organ_table_name().as_str());
        script.filter("record_id in (#{record_id})");
        script
    }
}
