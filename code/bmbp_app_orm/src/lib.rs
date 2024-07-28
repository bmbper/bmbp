use async_static::async_static;
use bmbp_app_vars::BmbpVar;
pub use bmbp_rdbc::*;
pub use bmbp_rdbc_orm::*;
async_static! {
    pub static ref RdbcOrmIns:RdbcOrm = build_orm().await;
}
async fn build_orm() -> RdbcOrm {
    let ds = build_datasource_with_vars();
    match RdbcOrm::new(ds).await {
        Ok(orm) => orm,
        Err(err) => {
            panic!("连接数据库失败:{:#?}", err);
        }
    }
}

fn build_datasource_with_vars() -> RdbcDataSource {
    let mut ds = RdbcDataSource::new();
    if let Some(bmbp) = BmbpVar.bmbp.as_ref() {
        if let Some(datasource) = bmbp.datasource.as_ref() {
            if let Some(driver_str) = datasource.driver.as_ref() {
                let driver = driver_str.to_string();
                match driver.as_str() {
                    "mysql" => {
                        ds.set_driver(RdbcDataBaseDriver::Mysql);
                    }
                    "postgresql" => {
                        ds.set_driver(RdbcDataBaseDriver::Postgres);
                    }
                    _ => {
                        panic!("不支持的数据库类型:{:#?}", driver);
                    }
                }
            }
            if let Some(host) = datasource.host.as_ref() {
                ds.set_host(host.to_string());
            }
            if let Some(port) = datasource.port.as_ref() {
                ds.set_port(port.clone() as u16);
            }
            if let Some(user) = datasource.username.as_ref() {
                ds.set_user(user.to_string());
            }
            if let Some(password) = datasource.password.as_ref() {
                ds.set_password(password.to_string());
            }
            if let Some(database) = datasource.database.as_ref() {
                ds.set_database(database.to_string());
            }
            if let Some(pool_size) = datasource.init.as_ref() {
                ds.set_init_conn_size(pool_size.clone() as usize);
            }
            if let Some(pool_size) = datasource.max_idle.as_ref() {
                ds.set_max_idle_conn(pool_size.clone() as usize);
            }
        }
    }
    ds
}
