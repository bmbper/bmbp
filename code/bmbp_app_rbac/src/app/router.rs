use crate::app::view::*;
use salvo::Router;

use super::web::*;

pub fn app_router() -> Router {
    let view_router = Router::with_path("/index.view").get(app_index_view);
    let api_router = Router::new()
        .push(Router::with_path("/find/page").post(find_app_page))
        .push(Router::with_path("/find/list").post(find_app_list))
        .push(Router::with_path("/find/info/<recordId>").get(find_app_info));

    Router::new().push(view_router).push(api_router)
}
