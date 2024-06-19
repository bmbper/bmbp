use crate::model::{BmbpDevDatasource, BmbpDevHttp, BmbpDevTable};
use salvo::Router;

pub fn builder_dev_manage_router() -> Router {
    let mut router = Router::with_path("/manage")
        .push(Router::with_path("/datasource").push(BmbpDevDatasource::build_router()))
        .push(Router::with_path("/table").push(BmbpDevTable::build_router()))
        .push(Router::with_path("http").push(BmbpDevHttp::build_router()));
    router
}
