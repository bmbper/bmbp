use crate::app::view::*;
use salvo::Router;

use super::web::find_app_page;

pub fn app_router() -> Router {
    let view_router = Router::with_path("/index.view").get(app_index_view);
    let api_router = Router::new().push(Router::with_path("/find/page").post(find_app_page));
    Router::new().push(view_router).push(api_router)
}
