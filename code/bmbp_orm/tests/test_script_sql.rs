use std::sync::Arc;

use serde::{Deserialize, Serialize};

use bmbp_orm::{BmbpDataSource, Orm};
use bmbp_types::BmbpResp;

#[derive(Clone, Serialize, Deserialize)]
pub struct DemoA {
    name: Option<String>,
    code: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DemoB {
    name: Option<String>,
    code: Option<usize>,
}

fn build_data_source() -> BmbpDataSource {
    let mut ds = BmbpDataSource::new();
    ds.set_host("127.0.0.1".to_string()).set_port(5432usize);
    ds.set_user("bmbp".to_string())
        .set_password("zgk0130!".to_string());
    ds.set_database("bmbp".to_string())
        .set_schema("public".to_string())
        .set_driver("postgres".to_string());
    ds
}

#[tokio::test]
async fn test_orm_query_struct() -> BmbpResp<()> {
    let ds = Arc::new(build_data_source());
    let orm = Orm::new(ds).await?;
    let sql = "select name,code from demo".to_string();
    let demo_a_lsit = orm.struct_query_list::<DemoA>(&sql).await?;
    assert_eq!(demo_a_lsit.len(), 2);
    Ok(())
}
