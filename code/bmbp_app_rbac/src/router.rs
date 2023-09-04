use crate::app::app_router;
use crate::app_menu::menu_router;
use crate::organ::organ_router;

use crate::role::role_router;
use crate::user::user_router;
use salvo::Router;
pub fn build_rbac_router() -> Router {
    Router::with_path("/rbac/v1")
        .push(Router::with_path("/organ").push(organ_router()))
        .push(Router::with_path("/user").push(user_router()))
        .push(Router::with_path("/app").push(app_router()))
        .push(Router::with_path("/role").push(role_router()))
        .push(Router::with_path("/menu").push(menu_router()))
}
