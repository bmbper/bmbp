use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;

use super::BmbpRbacMenu;

pub(crate) struct MenuScript();

impl MenuScript {
    /// 获取菜单机构信息表
    fn get_menu_table_name() -> String {
        BmbpRbacMenu::orm_table_name()
    }
    fn get_menu_table_columns() -> Vec<String> {
        BmbpRbacMenu::orm_table_column_name()
    }
}

impl MenuScript {
    /// 菜单机构查询语句
    pub(crate) fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_menu_table_name().as_str());
        let columns = Self::get_menu_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
    pub(crate) fn insert_script() -> BmbpScriptSql {
        let mut insert_script = BmbpScriptSql::new();
        insert_script.insert_into(Self::get_menu_table_name().as_str());
        for item in Self::get_menu_table_columns() {
            insert_script.insert_value(
                item.as_str(),
                format!("#{{{}}}", snake_to_camel(item.to_string())).as_str(),
            );
        }

        insert_script
    }
    pub(crate) fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_menu_table_name().as_str());
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
        script.delete(Self::get_menu_table_name().as_str());
        script
    }

    pub(crate) fn update_parent_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("menu_parent_code = #{menuParentCode}");
        script.filter("record_id = #{recordId}");
        script
    }
    pub(crate) fn update_title_path_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("menu_title_path = CONCAT( #{newMenuTitlePath}::TEXT,  RIGHT(ORGAN_TITLE_PATH,  LENGTH(ORGAN_TITLE_PATH) - LENGTH(#{oldMenuParentTitlePath})))");
        script.filter("menu_title_path like CONCAT(#{currentMenuTitlePath}::TEXT,'%')");

        script
    }
    pub(crate) fn update_code_path_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("menu_code_path = CONCAT( #{newMenuCodePath}::TEXT,  RIGHT(ORGAN_CODE_PATH,  LENGTH(ORGAN_CODE_PATH) - LENGTH(#{oldMenuParentCodePath})))");
        script.filter("menu_code_path like CONCAT(#{currentMenuCodePath}::TEXT,'%')");
        script
    }
}
