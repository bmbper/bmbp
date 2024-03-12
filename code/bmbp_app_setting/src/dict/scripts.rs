use bmbp_rdbc_orm::{Delete, Insert, Query, RDBC_DATA_ID, RDBC_DATA_STATUS, RdbcModel, RdbcTree, Update};
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
        insert.insert_op_column_value("dict_alise", dict_ext.get_dict_alias());
        insert.insert_op_column_value("dict_value", dict_ext.get_dict_value());
        insert.insert_op_column_value("dict_type", dict_ext.get_dict_type());
        insert
    }
    pub fn build_update(dict: &mut BmbpSettingDictOrmModel) -> Update {
        let mut update = Update::new();
        update.update_table(BmbpSettingDict::get_table_name());
        update.set_op("code", dict.get_code());
        update.set_op("code_path", dict.get_code_path());
        update.set_op("name", dict.get_name());
        update.set_op("name_path", dict.get_name_path());
        update.set_op("dict_type", dict.get_ext_props().get_dict_type());
        update.set_op("dict_alise", dict.get_ext_props().get_dict_alias());
        update.set_op("dict_value", dict.get_ext_props().get_dict_value());
        update.eq(RDBC_DATA_ID, dict.get_data_id().unwrap());
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
