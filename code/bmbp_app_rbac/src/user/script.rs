use bmbp_app_curd::CurdScript;
use bmbp_orm_ins::BmbpScriptSql;

use super::model::BmbpRbacUser;

pub struct UserScript;
impl UserScript {
    pub(crate) fn change_organ_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set_value("organ_id", "#{organId}");
        script.filter("record_id = #{recordId}");
        script
    }

    pub(crate) fn reset_password_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set_value("user_password", "#{password}");
        script.filter("record_id = #{recordId}");
        script
    }
    pub(crate) fn query_script_by_organ() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(format!("{} as u", Self::get_orm_table_name()).as_str());
        let columns = Self::get_orm_table_columns();
        for column in columns.as_slice() {
            if column.eq("user_password") {
                continue;
            }
            query_script.select_alias_with_table_alias(column, &"u".to_string());
        }
        query_script.select("org.organ_title_path as \"organTitlePath\"");
        query_script.from(" bmbp_rbac_organ as org ");
        query_script.filter("org.record_id = u.organ_id");
        query_script
    }
}

impl CurdScript for UserScript {
    fn get_orm_table_name() -> String {
        BmbpRbacUser::orm_table_name()
    }
    fn get_orm_table_columns() -> Vec<String> {
        BmbpRbacUser::orm_table_column_name()
    }
}
