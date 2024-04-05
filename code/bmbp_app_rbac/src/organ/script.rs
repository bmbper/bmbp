use bmbp_rdbc_orm::{
    Delete, Insert, Query, RDBC_DATA_ID, RDBC_DATA_STATUS, RdbcFilter, RdbcModel, RdbcTable, RdbcTree,
    Update,
};

use crate::organ::model::{BmbpRbacOrgan, BmbpRbacOrganTree};

pub struct BmbpRbacOrganScript;
impl BmbpRbacOrganScript {
    pub fn build_query_script() -> Query {
        let mut query = Query::new();
        let fields = BmbpRbacOrganTree::get_table_fields();
        for field in fields {
            query.select(field);
        }
        query.table(BmbpRbacOrganTree::get_table_name());
        query.order_by("data_sort", true);
        query
    }
    pub fn build_insert(organ: &BmbpRbacOrganTree) -> Insert {
        let mut insert = organ.build_insert();
        let organ_ext = organ.get_ext_props();
        if let Some(organ_type) = organ_ext.get_organ_type() {
            insert.insert_column_value("organ_type", organ_type.value());
        } else {
            insert.insert_column_value("organ_type", "");
        }

        insert
    }
    pub fn build_update(organ: &mut BmbpRbacOrganTree) -> Update {
        let mut update = Update::new();
        update.table(BmbpRbacOrgan::get_table_name());
        update.set("name", organ.get_name());
        update.set("name_path", organ.get_name_path());
        update.set("data_sort", organ.get_data_sort());
        update.eq_(RDBC_DATA_ID, organ.get_data_id().unwrap());
        update
    }
    pub fn build_update_status(code_path: &String, status: &str) -> Update {
        let mut update = Update::new();
        update
            .table(BmbpRbacOrgan::get_table_name())
            .set(RDBC_DATA_STATUS, status)
            .like_left_value("code_path", code_path);
        update
    }
    pub fn build_delete_script(organ_id: Option<String>) -> Delete {
        let mut delete_organ = Delete::new();
        delete_organ
            .table(BmbpRbacOrgan::get_table_name())
            .eq_(BmbpRbacOrgan::get_table_primary_key(), organ_id.unwrap());
        delete_organ
    }
}
