use async_trait::async_trait;
#[async_trait]
pub trait RdbcDbConn {
    async fn is_valid(&self) -> bool;
}

