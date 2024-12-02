use bmbp_sql::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper};

pub trait BmbpOrmTable {
    fn table() -> String;
    fn columns() -> Vec<String>;
    fn primary_key() -> String {
        "data_id".to_string()
    }
    fn primary_key_list() -> Vec<String> {
        vec![]
    }

    fn list_script_wrapper() -> RdbcQueryWrapper;
    fn info_script_wrapper() -> RdbcQueryWrapper;
    fn insert_script_wrapper() -> RdbcInsertWrapper;
    fn update_script_wrapper() -> RdbcInsertWrapper;
    fn delete_script_wrapper() -> RdbcInsertWrapper;
    fn info_id_script_wrapper() -> RdbcQueryWrapper;
    fn delete_id_script_wrapper() -> RdbcDeleteWrapper;

    fn list_wrapper(&self, nullable: bool) -> RdbcQueryWrapper;
    fn info_wrapper(&self, nullable: bool) -> RdbcQueryWrapper;
    fn insert_wrapper(&self, nullable: bool) -> RdbcInsertWrapper;
    fn update_wrapper(&self, nullable: bool) -> RdbcInsertWrapper;
    fn delete_wrapper(&self, nullable: bool) -> RdbcInsertWrapper;
    fn info_id_wrapper(&self, data_id: String) -> RdbcQueryWrapper;
    fn delete_id_wrapper(&self, data_id: String) -> RdbcDeleteWrapper;
}
