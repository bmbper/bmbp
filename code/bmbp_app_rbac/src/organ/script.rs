use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;

use super::BmbpRbacOrgan;

pub(crate) struct OrganScript();

impl OrganScript {
    /// 获取组织机构信息表
    fn get_organ_table_name() -> String {
        BmbpRbacOrgan::orm_table_name()
    }
    fn get_organ_table_columns() -> Vec<String> {
        BmbpRbacOrgan::orm_table_column_name()
    }
}

impl OrganScript {
    /// 组织机构查询语句
    pub(crate) fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_organ_table_name().as_str());
        let columns = Self::get_organ_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
    pub(crate) fn insert_script() -> BmbpScriptSql {
        let mut insert_script = BmbpScriptSql::new();
        insert_script.insert_into(Self::get_organ_table_name().as_str());
        for item in Self::get_organ_table_columns() {
            insert_script.insert_value(
                item.as_str(),
                format!("#{{{}}}", snake_to_camel(item.to_string())).as_str(),
            );
        }

        insert_script
    }
    pub(crate) fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_organ_table_name().as_str());
        update
    }
    pub(crate) fn update_status_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("record_status = #{recordStatus}");
        script.filter("record_id = #{recordId}");
        script
    }
    pub(crate) fn update_parent_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("organ_parent_code = #{organ_parent_code}");
        script.filter("record_id = #{recordId}");
        script
    }
    pub(crate) fn delete_script_by_id() -> BmbpScriptSql {
        let mut script = Self::delete_script();
        script.filter("record_id in (#{recordId})");
        script
    }
    pub(crate) fn delete_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.delete(Self::get_organ_table_name().as_str());
        script.filter("record_id in (#{recordId})");
        script
    }
}
