use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;

use super::BmbpRbacRole;

pub(crate) struct RoleScript();

impl RoleScript {
    /// 获取组织机构信息表
    fn get_role_table_name() -> String {
        BmbpRbacRole::orm_table_name()
    }
    fn get_role_table_columns() -> Vec<String> {
        BmbpRbacRole::orm_table_column_name()
    }
}

impl RoleScript {
    /// 组织机构查询语句
    pub(crate) fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_role_table_name().as_str());
        let columns = Self::get_role_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
    pub(crate) fn insert_script() -> BmbpScriptSql {
        let mut insert_script = BmbpScriptSql::new();
        insert_script.insert_into(Self::get_role_table_name().as_str());
        for item in Self::get_role_table_columns() {
            insert_script.insert_value(
                item.as_str(),
                format!("#{{{}}}", snake_to_camel(item.to_string())).as_str(),
            );
        }

        insert_script
    }
    pub(crate) fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_role_table_name().as_str());
        update
    }
    pub(crate) fn update_status_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("record_status = #{recordStatus}");
        script
    }

    pub(crate) fn delete_script_by_id() -> BmbpScriptSql {
        let mut delete_script = Self::delete_script();
        delete_script.filter("record_id = #{recordId}");
        delete_script
    }
    pub(crate) fn delete_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.delete(Self::get_role_table_name().as_str());
        script
    }

    pub(crate) fn update_parent_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("role_parent_code = #{roleParentCode}");
        script.filter("record_id = #{recordId}");
        script
    }
    pub(crate) fn update_title_path_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("role_title_path = CONCAT( #{newRoleTitlePath}::TEXT,  RIGHT(ORGAN_TITLE_PATH,  LENGTH(ORGAN_TITLE_PATH) - LENGTH(#{oldRoleParentTitlePath})))");
        script.filter("role_title_path like CONCAT(#{currentRoleTitlePath}::TEXT,'%')");

        script
    }
    pub(crate) fn update_code_path_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("role_code_path = CONCAT( #{newRoleCodePath}::TEXT,  RIGHT(ORGAN_CODE_PATH,  LENGTH(ORGAN_CODE_PATH) - LENGTH(#{oldRoleParentCodePath})))");
        script.filter("role_code_path like CONCAT(#{currentRoleCodePath}::TEXT,'%')");
        script
    }
}
