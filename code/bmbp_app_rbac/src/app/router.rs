use salvo::Router;

pub fn app_router() -> Router {
    Router::new()
        .push(Router::with_path("/page").post())
        .push(Router::with_path("/list").post())
        .push(Router::with_path("/save").post())
        .push(Router::with_path("/info/id/<dataId>").post())
        .push(Router::with_path("/enable/<dataId>").post())
        .push(Router::with_path("/disable/<dataId>").post())
        .push(Router::with_path("/remove/<dataId>").post())
}
