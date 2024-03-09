use bmbp_rdbc_orm::{Delete, Insert, Query, RDBC_DATA_ID, RDBC_DATA_STATUS, RdbcModel, Update};
use crate::dict::model::{BmbpSettingDict, BmbpSettingDictOrmModel};

pub struct BmbpRdbcDictScript;

impl BmbpRdbcDictScript {
    pub fn build_query_script() -> Query {
        let mut query = Query::new();
        let fields = BmbpSettingDictOrmModel::get_table_fields();
        for field in fields {
            query.select(field);
        }
        query.query_table(BmbpSettingDictOrmModel::get_table_name());
        query.order_by("data_sort", true);
        query
    }
    pub fn build_insert(dict: &BmbpSettingDictOrmModel) -> Insert {
        let mut insert = dict.build_insert();
        let dict_ext = dict.get_ext_props();
        insert.insert_column_value("dict_alise", dict_ext.get_dict_alise().clone());
        insert.insert_column_value("dict_value", dict_ext.get_dict_value().clone());
        insert.insert_column_value("dict_type", dict_ext.get_dict_type().value());
        insert
    }
    pub fn build_update(_dict: &BmbpSettingDictOrmModel) -> Update {
        let mut update = Update::new();
        update.update_table(BmbpSettingDict::get_table_name());
        update.set_("dict_value", _dict.get_ext_props().get_dict_value());
        update.eq(RDBC_DATA_ID, _dict.get_data_id());
        update
    }
    pub fn build_update_status(dict_id: Option<String>, status: i8) -> Update {
        let mut update = Update::new();
        update.update_table(BmbpSettingDict::get_table_name()).set(RDBC_DATA_STATUS, status).eq(BmbpSettingDict::get_table_primary_key(), dict_id.unwrap());
        update
    }
    pub fn build_delete_script(dict_id: Option<String>) -> Delete {
        let mut delete_dict = Delete::new();
        delete_dict.delete_table(BmbpSettingDict::get_table_name()).eq(BmbpSettingDict::get_table_primary_key(), dict_id.unwrap());
        delete_dict
    }
}
