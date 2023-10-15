use bmbp_app_curd::CurdScript;
use bmbp_orm_ins::BmbpScriptSql;

use crate::role::BmbpRbacRole;

use super::model::BmbpRbacRoleUser;

pub struct RbacRoleUserScript;
impl CurdScript for RbacRoleUserScript {
    fn get_orm_table_name() -> String {
        BmbpRbacRoleUser::orm_table_name()
    }

    fn get_orm_table_columns() -> Vec<String> {
        BmbpRbacRoleUser::orm_table_column_name()
    }
}

impl RbacRoleUserScript {
    pub(crate) fn query_role_checked_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.select_alias("role_id");
        query_script.from(&BmbpRbacRoleUser::orm_table_name());
        query_script.filter("user_id = #{userId}");
        query_script
    }

    pub(crate) fn query_role_by_user_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        let role_columns = BmbpRbacRoleUser::orm_table_column_name();
        for item in role_columns.as_slice() {
            if item.eq_ignore_ascii_case("record_status") {
                continue;
            }
            query_script.select_alias_with_table_alias(item.as_str(), &"u2".to_string());
        }
        query_script.select_alias_with_table_alias("role_title", &"r1".to_string());
        query_script.select_alias_with_table_alias("role_title_path", &"r1".to_string());
        query_script.select_alias_with_table_alias("record_status", &"r1".to_string());
        query_script.select_alias_with_table_alias("role_type", &"r1".to_string());

        query_script.from_alias(&BmbpRbacRole::orm_table_name(), "r1");
        query_script.from_alias(&BmbpRbacRoleUser::orm_table_name(), "u2");
        query_script.filter("r1.record_id = u2.role_id");
        query_script.filter("u2.user_id = #{userId}");
        query_script
    }

    pub(crate) fn delete_script_by_user() -> BmbpScriptSql {
        let mut del_script = Self::delete_script();
        del_script.filter("user_id = #{userId}");
        del_script
    }
}
