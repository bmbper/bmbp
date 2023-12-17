use bmbp_app_common::BmbpCurdModel;
use bmbp_app_utils::snake_to_camel;
use bmbp_orm_ins::BmbpScriptSql;
use bmbp_orm_sql::{DeleteBuilder, InsertBuilder, QueryBuilder, UpdateBuilder};

pub trait CurdScript {
    fn get_orm_table_name() -> String;
    fn get_orm_table_columns() -> Vec<String>;
    fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_orm_table_name().as_str());
        let columns = Self::get_orm_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script
    }
    fn query_info_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(Self::get_orm_table_name().as_str());
        let columns = Self::get_orm_table_columns();
        query_script.select_slice_alias(columns.as_slice());
        query_script.filter("record_id = #{recordId}");
        query_script
    }
    fn insert_script() -> BmbpScriptSql {
        let mut insert_script = BmbpScriptSql::new();
        insert_script.insert_into(Self::get_orm_table_name().as_str());
        for item in Self::get_orm_table_columns() {
            insert_script.insert_value(
                item.as_str(),
                format!("#{{{}}}", snake_to_camel(item.to_string())).as_str(),
            );
        }

        insert_script
    }
    fn update_script() -> BmbpScriptSql {
        let mut update = BmbpScriptSql::new();
        update.update(Self::get_orm_table_name().as_str());
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
        script.delete(Self::get_orm_table_name().as_str());
        script
    }
    fn delete_script_by_id() -> BmbpScriptSql {
        let mut delete_script = Self::delete_script();
        delete_script.filter("record_id = #{recordId}");
        delete_script
    }
}

pub struct BmbpCurdScript {}

impl BmbpCurdScript {
    pub fn query<T>() -> QueryBuilder where T: BmbpCurdModel {
        let mut query = QueryBuilder::new();
        T::get_table_columns().iter().for_each(|field| {
            query.select(field);
        });
        query.from(T::get_table_name().as_str());
        query
    }
    pub fn query_info<T>() -> QueryBuilder where T: BmbpCurdModel {
        let mut query = Self::query::<T>();
        query.eq(T::get_table_primary_key().as_str(), "#{record_id}");
        query
    }

    pub fn insert<T>() -> InsertBuilder where T: BmbpCurdModel {
        let mut insert = InsertBuilder::new();
        insert.into_table(T::get_table_name().as_str());
        T::get_table_columns().iter().for_each(|field| {
            insert.field_value(field, format!("#{{{}}}", field.to_string()).as_str());
        });
        insert
    }
    pub fn update<T>() -> UpdateBuilder where T: BmbpCurdModel {
        let mut update = UpdateBuilder::new();
        update.from(T::get_table_name().as_str());
        T::get_table_columns().iter().for_each(|field| {
            update.set(field, format!("#{{{}}}", field.to_string()).as_str());
        });
        update.eq(T::get_table_primary_key().as_str(), "#{record_id}");
        update
    }
    pub fn delete<T>() -> DeleteBuilder where T: BmbpCurdModel {
        let mut delete = DeleteBuilder::new();
        delete.from(T::get_table_name().as_str());
        delete.eq(T::get_table_primary_key().as_str(), "#{record_id}");
        delete
    }
    pub fn delete_logic<T>() -> UpdateBuilder where T: BmbpCurdModel {
        let mut update = UpdateBuilder::new();
        update.from(T::get_table_name().as_str());
        update.set("record_flag", "'-1'");
        update.eq(T::get_table_primary_key().as_str(), "#{record_id}");
        update
    }
    pub fn enable<T>() -> UpdateBuilder where T: BmbpCurdModel {
        Self::update_status::<T>("'0'")
    }
    pub fn disable<T>() -> UpdateBuilder where T: BmbpCurdModel {
        Self::update_status::<T>("'-1'")
    }
    pub fn update_status<T>(status: &str) -> UpdateBuilder where T: BmbpCurdModel {
        let mut update = UpdateBuilder::new();
        update.from(T::get_table_name().as_str());
        update.set("record_status", status);
        update.eq(T::get_table_primary_key().as_str(), "#{record_id}");
        update
    }
}
