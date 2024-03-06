use bmbp_rdbc_orm::{Delete, Insert, Query, RDBC_DATA_STATUS, RdbcModel, Update};
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
    pub fn build_insert(_dict: &BmbpSettingDictOrmModel) -> Insert {
        Insert::new()
    }
    pub fn build_update(_dict: &BmbpSettingDictOrmModel) -> Update {
        Update::new()
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
