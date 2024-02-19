use salvo::Router;
use crate::login::login;

pub fn build_home_router() -> Router {
    let api_router = Router::new()
        .push(Router::with_path("/api/v1").push(Router::with_path("/login").post(login)));
    Router::new().push(api_router)
}
