use async_static::async_static;

pub use crate::err::RdbcResult;
use bmbp_app_common::map::{
    global_hash_map_vars, global_hash_map_vars_to_bool, global_hash_map_vars_to_usize,
};
pub use bmbp_rdbc_model::*;
pub use bmbp_rdbc_sql::*;
pub use ds::*;
pub use err::*;
pub use orm::RdbcOrm;
pub use orm::*;

mod client;
mod ds;
mod err;
mod orm;
mod pool;
mod val;

async_static! {
    pub static ref RdbcOrmIns:RdbcOrm = build_orm().await;
}
async fn build_orm() -> RdbcOrm {
    let ds_rs = build_data_source_from_vars();
    match ds_rs {
        Ok(ds) => match RdbcOrm::new(ds).await {
            Ok(orm) => orm,
            Err(err) => {
                panic!("连接数据库失败:{:#?}", err);
            }
        },
        Err(err) => {
            panic!("不支持的数据库类型:{:#?}", err);
        }
    }
}
fn build_data_source_from_vars() -> RdbcResult<RdbcDataSource> {
    let mut data_source = RdbcDataSource::new();
    let driver = global_hash_map_vars("bmbp_ds".to_string(), "ds_driver".to_string());
    data_source.set_driver(RdbcDataBaseDriver::value_of(driver)?);
    data_source
        .set_host(global_hash_map_vars(
            "bmbp_ds".to_string(),
            "ds_host".to_string(),
        ))
        .set_port(
            global_hash_map_vars_to_usize("bmbp_ds".to_string(), "ds_port".to_string()) as u16,
        )
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
    Ok(data_source)
}

#[cfg(test)]
pub mod tests {
    use crate::{RdbcDataBaseDriver, RdbcDataSource, RdbcOrm};

    fn build_datasource() -> RdbcDataSource {
        let mut ds = RdbcDataSource::new();
        ds.set_driver(RdbcDataBaseDriver::Postgres);
        ds.set_host("127.0.0.1".to_string())
            .set_port(5432)
            .set_user("bmbp".to_string())
            .set_password("zgk0130!".to_string())
            .set_database("bmbp".to_string())
            .set_schema("public".to_string())
            .set_ignore_case(true);
        ds.set_init_conn_size(5)
            .set_max_conn_size(10)
            .set_max_idle_conn(1);
        ds
    }

    #[tokio::test]
    async fn test_rom() {
        let ds = build_datasource();
        if let Ok(orm) = RdbcOrm::new(ds).await {
            if let Ok(conn) = orm.get_conn().await {
                assert!(conn.valid().await);
            };
        }
    }
}
