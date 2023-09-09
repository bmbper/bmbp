use crate::app::app_router;
use crate::app_data_scope::app_data_scope_router;
use crate::app_menu::menu_router;
use crate::app_url::app_url_router;
use crate::organ::organ_router;

use crate::role::role_router;
use crate::role_app::role_app_router;
use crate::role_app_data_scope::role_app_data_scope_router;
use crate::role_app_menu::role_app_menu_router;
use crate::role_app_url::role_app_url_router;
use crate::role_user::role_user_router;
use crate::user::user_router;
use salvo::Router;
pub fn build_rbac_router() -> Router {
    Router::with_path("/rbac/v1")
        .push(Router::with_path("/organ").push(organ_router()))
        .push(Router::with_path("/user").push(user_router()))
        .push(Router::with_path("/app").push(app_router()))
        .push(Router::with_path("/menu").push(menu_router()))
        .push(Router::with_path("/app/url").push(app_url_router()))
        .push(Router::with_path("/app/data/scope").push(app_data_scope_router()))
        .push(Router::with_path("/role").push(role_router()))
        .push(Router::with_path("/role/user").push(role_user_router()))
        .push(Router::with_path("/role/app").push(role_app_router()))
        .push(Router::with_path("/role/url").push(role_app_url_router()))
        .push(Router::with_path("/role/app/menu").push(role_app_menu_router()))
        .push(Router::with_path("/role/app/data/scope").push(role_app_data_scope_router()))
}
