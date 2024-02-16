mod conn;
mod datasource;
mod orm;
mod pool;
mod client;
mod err;

pub use datasource::*;
pub use orm::*;

#[cfg(test)]
pub mod tests {
    use crate::{RdbcDataBaseDriver, RdbcDataSource, RdbcOrm};

    fn build_datasource() -> RdbcDataSource {
        let mut ds = RdbcDataSource::new(RdbcDataBaseDriver::Postgres);
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
        let orm = RdbcOrm::new(ds).await;
        let conn = orm.get_conn().await;
        assert!(conn.is_valid().await)
    }
}
