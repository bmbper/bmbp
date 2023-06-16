use crate::user::web::*;
use axum::{routing::post, Router};

pub fn user_router() -> Router {
    Router::new().route("/user/tree", post(find_user_tree))
}
