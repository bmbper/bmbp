use bmbp_app_curd::CurdScript;
use bmbp_orm_ins::BmbpScriptSql;

use super::model::BmbpRbacUser;

pub struct UserScript;
impl UserScript {
    pub(crate) fn change_organ_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.update(Self::get_organ_table_name().as_str());
        script.set_value("organ_code", "#{organCode}");
        script.filter("record_id = #{recordId}");
        script
    }

    pub(crate) fn reset_password_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.update(Self::get_organ_table_name().as_str());
        script.set_value("user_password", "#{password}");
        script.filter("record_id = #{recordId}");
        script
    }
}

impl CurdScript for UserScript {
    fn get_organ_table_name() -> String {
        BmbpRbacUser::orm_table_name()
    }
    fn get_organ_table_columns() -> Vec<String> {
        BmbpRbacUser::orm_table_column_name()
    }
}
