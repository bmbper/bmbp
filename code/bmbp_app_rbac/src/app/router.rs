use crate::app::view::*;
use salvo::Router;
pub fn app_router() -> Router {
    Router::new().push(Router::with_path("/index.view").get(app_index_view))
}
