use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;

pub trait CurdScript {
    fn get_organ_table_name() -> String;
    fn get_organ_table_columns() -> Vec<String>;
    fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_organ_table_name().as_str());
        let columns = Self::get_organ_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
    fn query_info_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_organ_table_name().as_str());
        let columns = Self::get_organ_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script.filter("record_id = #{recordId}");
        query_script
    }
    fn insert_script() -> BmbpScriptSql {
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
    fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_organ_table_name().as_str());
        update.set_value("record_update_user", "#{recordUpdateUser}");
        update.set_value("record_update_time", "#{recordUpdateTime}");
        update
    }
    fn update_status_script() -> BmbpScriptSql {
        let mut script = Self::update_script();
        script.set("record_status = #{recordStatus}");
        script.filter("record_id = #{recordId}");
        script
    }
    fn delete_script() -> BmbpScriptSql {
        let mut script = BmbpScriptSql::new();
        script.delete(Self::get_organ_table_name().as_str());
        script
    }
    fn delete_script_by_id() -> BmbpScriptSql {
        let mut delete_script = Self::delete_script();
        delete_script.filter("record_id = #{recordId}");
        delete_script
    }
}
