use crate::role::model::BmbpRbacRole;
use salvo::Router;

pub fn role_router() -> Router {
    let mut router = BmbpRbacRole::build_router();
    router
}
