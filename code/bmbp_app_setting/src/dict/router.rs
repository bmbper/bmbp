use salvo::Router;
use crate::dict::web::*;

pub fn build_dict_router() -> Router {
    let mut dict_router = Router::with_path("/dict");
    dict_router = dict_router
        .push(Router::with_path("/tree").post(find_dict_tree))
        .push(Router::with_path("/page").post(find_dict_page))
        .push(Router::with_path("/list").post(find_dict_list))
        .push(Router::with_path("/save").post(save_dict))
        .push(Router::with_path("/insert").post(insert_dict))
        .push(Router::with_path("/update").post(update_dict))
        .push(Router::with_path("/info/<recordId>").post(find_dict_info).get(find_dict_info))
        .push(Router::with_path("/enable/<recordId>").post(enable_dict))
        .push(Router::with_path("/disable/<recordId>").post(disable_dict))
        .push(Router::with_path("/delete/<recordId>").post(delete_dict))
        .push(Router::with_path("/change/parent/tree/<recordId>").post(find_dict_tree_exclude_by_id))
        .push(Router::with_path("/change/parent/save/<recordId>").post(save_dict_parent));
    dict_router
}