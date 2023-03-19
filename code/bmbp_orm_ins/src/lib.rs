use std::sync::Arc;

use async_static::async_static;

use bmbp_orm::BmbpDataSource;
use bmbp_orm::Orm;
use bmbp_vars::map::{
    global_hash_map_vars, global_hash_map_vars_to_bool, global_hash_map_vars_to_usize,
};

async_static! {
 pub static  ref BmbpORM:Arc<Orm> = build_orm_ins().await;
}
async fn build_orm_ins() -> Arc<Orm> {
    let data_source = build_data_source_from_vars();
    let arc_data_source = Arc::new(data_source);
    let orm_rs = Orm::new(arc_data_source.clone()).await;
    match orm_rs {
        Ok(orm) => orm.clone(),
        Err(e) => {
            panic!("数据库初始化连接错误:{}", e.to_string())
        }
    }
}

fn build_data_source_from_vars() -> BmbpDataSource {
    let mut data_source = BmbpDataSource::new();
    data_source
        .set_driver(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_driver".to_string(),
        ))
        .set_host(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_host".to_string(),
        ))
        .set_port(global_hash_map_vars_to_usize(
            "bmbp_ds".to_string(),
            "ds_port".to_string(),
        ))
        .set_database(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_database".to_string(),
        ))
        .set_schema(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_schema".to_string(),
        ))
        .set_user(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_user".to_string(),
        ))
        .set_password(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_password".to_string(),
        ))
        .set_ignore_case(global_hash_map_vars_to_bool(
            "bmbp_ds".to_string(),
            "ds_ignore_case".to_string(),
        ));
    data_source
}

pub use bmbp_orm::BmbpDeleteSQL;
pub use bmbp_orm::BmbpDynamicSQL;
pub use bmbp_orm::BmbpInsertSQL;
pub use bmbp_orm::BmbpOrmSQL;
pub use bmbp_orm::BmbpQuerySQL;
pub use bmbp_orm::BmbpScriptSql;
pub use bmbp_orm::BmbpUpdateSQL;
