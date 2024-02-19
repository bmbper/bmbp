use salvo::Router;
use crate::index::index;

pub fn build_app_base_router() -> Router {
    Router::new().get(index)
}
