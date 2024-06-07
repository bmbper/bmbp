use crate::api::execute_api;
use crate::view::query_view_config;
use salvo::Router;

pub fn builder_dev_runtime_router() -> Router {
    let mut router = Router::with_path("/runtime")
        .push(
            Router::with_path("/api/<apiCode>")
                .get(execute_api)
                .post(execute_api)
                .delete(execute_api)
                .put(execute_api),
        )
        .push(
            Router::with_path("/view/<viewCode>")
                .get(query_view_config)
                .post(query_view_config),
        );
    router
}
