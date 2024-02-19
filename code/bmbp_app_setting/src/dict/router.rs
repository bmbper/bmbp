use salvo::Router;
use crate::dict::web::{delete_dict, disable_dict, enable_dict, find_dict_info, find_dict_list, find_dict_page, find_dict_tree, find_dict_tree_exclude_by_id, insert_dict, save_dict, save_dict_parent, update_dict};

pub fn build_dict_router() -> Router {
    let mut dict_router = Router::with_path("/dict");
    dict_router = dict_router
        .push(Router::with_path("/tree").post(find_dict_tree))
        .push(Router::with_path("/page").post(find_dict_page))
        .push(Router::with_path("/list").post(find_dict_list))
        .push(Router::with_path("/save").post(save_dict))
        .push(Router::with_path("/insert").post(insert_dict))
        .push(Router::with_path("/update").post(update_dict))
        .push(Router::with_path("/info/<record_id>").post(find_dict_info).get(find_dict_info))
        .push(Router::with_path("/enable/<record_id>").post(enable_dict))
        .push(Router::with_path("/disable/<record_id>").post(disable_dict))
        .push(Router::with_path("/delete/<record_id>").post(delete_dict))
        .push(Router::with_path("/change/parent/tree/<id>").post(find_dict_tree_exclude_by_id))
        .push(Router::with_path("/change/parent/save/<id>").post(save_dict_parent));
    dict_router
}