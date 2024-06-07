use crate::model::{BmbpDevHttp, BmbpDevTable};
use salvo::Router;

pub fn builder_dev_manage_router() -> Router {
    let mut router = Router::with_path("/manage")
        .push(BmbpDevTable::build_router())
        .push(BmbpDevHttp::build_router());
    router
}
