mod conn;
mod datasource;
mod orm;
mod pool;
mod client;

pub use datasource::*;
pub use orm::*;

#[cfg(test)]
pub mod tests {
    use crate::{orm, RdbcDataSource};

    #[tokio::test]
    async fn test_rom() {
        orm::RdbcOrm::new(RdbcDataSource::new()).await;
        assert!(true);
    }
}
