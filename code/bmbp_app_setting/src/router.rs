use crate::dict::build_dict_router;
use salvo::Router;

pub fn build_setting_router() -> Router {
    let mut setting_router = Router::with_path("setting");
    setting_router = setting_router.push(Router::with_path("v1").push(build_dict_router()));
    setting_router
}
