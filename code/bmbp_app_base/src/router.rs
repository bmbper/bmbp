use crate::index::index;
use salvo::Router;

pub fn build_app_base_router() -> Router {
    Router::new().get(index)
}
