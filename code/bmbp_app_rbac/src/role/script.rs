use bmbp_rdbc_orm::{
    Delete, Insert, Query, RDBC_DATA_ID, RDBC_DATA_STATUS, RdbcFilter, RdbcModel, RdbcTable, Update,
};

use crate::role::model::BmbpRbacRoleModel;

pub struct BmbpRbacRoleScript;
impl BmbpRbacRoleScript {
    pub fn build_query_script() -> Query {
        let mut query = Query::new();
        let fields = BmbpRbacRoleModel::get_table_fields();
        for field in fields {
            query.select(field);
        }
        query.table(BmbpRbacRoleModel::get_table_name());
        query.order_by("data_sort", true);
        query
    }
    pub fn build_insert(role: &BmbpRbacRoleModel) -> Insert {
        let mut insert = role.build_insert();
        let organ_ext = role.get_ext_props();
        if let Some(data) = organ_ext.get_role_code() {
            insert.insert_column_value("role_code", data);
        } else {
            insert.insert_column_value("role_code", "");
        }
        if let Some(data) = organ_ext.get_role_name() {
            insert.insert_column_value("role_name", data);
        } else {
            insert.insert_column_value("role_name", "");
        }
        if let Some(data) = organ_ext.get_role_desc() {
            insert.insert_column_value("role_desc", data);
        } else {
            insert.insert_column_value("role_desc", "");
        }

        insert
    }
    pub fn build_update(role: &mut BmbpRbacRoleModel) -> Update {
        let mut update = Update::new();
        update.table(BmbpRbacRoleModel::get_table_name());
        update.set("role_name", role.get_ext_props().get_role_name());
        update.set("role_desc", role.get_ext_props().get_role_desc());
        update.set("data_sort", role.get_data_sort());
        update.eq_(RDBC_DATA_ID, role.get_data_id().unwrap());
        update
    }
    pub fn build_update_status(data_id: &String, status: &str) -> Update {
        let mut update = Update::new();
        update
            .table(BmbpRbacRoleModel::get_table_name())
            .set(RDBC_DATA_STATUS, status)
            .eq_("data_id", data_id);
        update
    }
    pub fn build_delete_script(organ_id: Option<String>) -> Delete {
        let mut delete_organ = Delete::new();
        delete_organ.table(BmbpRbacRoleModel::get_table_name()).eq_(
            BmbpRbacRoleModel::get_table_primary_key(),
            organ_id.unwrap(),
        );
        delete_organ
    }
}
