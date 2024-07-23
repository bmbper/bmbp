use crate::meta::ddl::factory::DdlBuilderFactory;
use crate::{BmbpDBType, BmbpRdbcTable};

pub struct DdlClient;
impl DdlClient {
    pub fn build_create_table_sql(
        db_type: &BmbpDBType,
        table: &BmbpRdbcTable,
    ) -> Result<String, String> {
        let builder = DdlBuilderFactory::get_ddl_builder(db_type);
        if builder.is_none() {
            return Err("不支持的数据库类型".to_string());
        }
        builder.unwrap().build_create_table_sql(table)
    }
}
