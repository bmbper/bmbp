use bmbp_orm::{RdbcDataSource, RdbcDbType, RdbcOrm};
use std::sync::{Arc, LazyLock, RwLock};

pub static BMBP_APP_ORM: LazyLock<RwLock<RdbcOrm>> = LazyLock::new(|| {
    // TODO BMBP_VARS
    let ds = RdbcDataSource {
        db_type: RdbcDbType::Mysql,
        host: "".to_string(),
        port: 0,
        user: "".to_string(),
        password: "".to_string(),
        db_name: "".to_string(),
        charset: "".to_string(),
        pool_config: Default::default(),
    };
    RwLock::new(RdbcOrm::new(Arc::new(ds)).unwrap())
});
