use axum::Router;

use static_file::build_static_method_serve;

mod static_file;

pub fn build_file_router() -> Router {
    let mut router = Router::new();
    let serve_dir_router = build_static_method_serve();
    router = router.nest_service("/webapp/lib", serve_dir_router);
    router
}
