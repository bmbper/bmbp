use super::model::BmbpRbacApp;
use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;

/// RbacAppScript  应用的SQL脚本
pub struct RbacAppScript;

impl RbacAppScript {
    pub fn get_table_name() -> String {
        BmbpRbacApp::orm_table_name()
    }
    pub fn get_table_columns() -> Vec<String> {
        BmbpRbacApp::orm_table_column_name()
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

    pub(crate) fn update_script_for_status() -> BmbpScriptSql {
        let mut update_scirpt = BmbpScriptSql::new();
        update_scirpt.update(Self::get_table_name().as_str());
        update_scirpt.set_value("record_status", "#{recordStatus}");
        update_scirpt.filter("record_id = #{recordId}");
        update_scirpt
    }

    pub(crate) fn delete_script_by_reocrd_id() -> BmbpScriptSql {
        let mut delete_scirpt = BmbpScriptSql::new();
        delete_scirpt.delete(Self::get_table_name().as_str());
        delete_scirpt.filter("record_id = #{recordId}");
        delete_scirpt
    }
    pub(crate) fn insert_script() -> BmbpScriptSql {
        let mut insert_script = BmbpScriptSql::new();
        insert_script.insert_into(Self::get_table_name().as_str());
        for item in Self::get_table_columns() {
            insert_script.insert_value(
                item.as_str(),
                format!("#{{{}}}", snake_to_camel(item.to_string())).as_str(),
            );
        }

        insert_script
    }
}
