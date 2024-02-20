use bmbp_rdbc_orm::{Query, RdbcModel};
use crate::dict::model::BmbpSettingDictOrmModel;

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