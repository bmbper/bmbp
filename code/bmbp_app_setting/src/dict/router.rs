use salvo::Router;
use crate::dict::web::find_dict_tree;

pub fn build_dict_router() -> Router {
    let mut dict_router = Router::with_path("/dict");
    dict_router = dict_router.push(Router::with_path("/tree").post(find_dict_tree));
    dict_router
}