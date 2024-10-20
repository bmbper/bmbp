use crate::user::model::BmbpRbacUser;
use salvo::Router;

pub fn user_router() -> Router {
    BmbpRbacUser::build_router()
}
