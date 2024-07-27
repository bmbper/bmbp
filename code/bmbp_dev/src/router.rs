use bmbp_dev_manage::builder_dev_manage_router;
use bmbp_dev_runtime::builder_dev_runtime_router;
use salvo::Router;

pub fn builder_dev_router() -> Router {
    let router = Router::with_path("bmbp/dev")
        .push(builder_dev_runtime_router())
        .push(builder_dev_manage_router());
    router
}
