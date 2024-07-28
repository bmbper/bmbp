use bmbp_app_vars::BmbpVar;

async_static! {
    pub static ref RdbcOrmIns:RdbcOrm = build_orm().await;
}
async fn build_orm() -> RdbcOrm {
    let ds_rs = RdbcDataSource::new();
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
