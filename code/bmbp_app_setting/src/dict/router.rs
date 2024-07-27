use salvo::Router;

use crate::dict::web::*;

pub fn build_dict_router() -> Router {
    let mut dict_router = Router::with_path("dict");
    dict_router = dict_router
        .push(Router::with_path("tree").post(find_dict_tree))
        .push(Router::with_path("page").post(find_dict_page))
        .push(Router::with_path("list").post(find_dict_list))
        .push(Router::with_path("save").post(save_dict))
        .push(Router::with_path("insert").post(insert_dict))
        .push(Router::with_path("update").post(update_dict))
        .push(
            Router::with_path("info/<dataId>")
                .post(find_dict_info)
                .get(find_dict_info),
        )
        .push(Router::with_path("enable/<dataId>").post(enable_dict))
        .push(Router::with_path("disable/<dataId>").post(disable_dict))
        .push(Router::with_path("delete/<dataId>").post(delete_dict))
        .push(Router::with_path("change/parent/tree/<dataId>").post(find_dict_tree_exclude_by_id))
        .push(Router::with_path("change/parent/save/<dataId>").post(save_dict_parent))
        .push(Router::with_path("combo/alias/<alias>").post(find_combo_by_alias))
        .push(Router::with_path("combo/code/<code>").post(find_combo_by_code))
        .push(Router::with_path("combo/id/<dataId>").post(find_combo_by_id))
        .push(Router::with_path("cascade/combo/alias/<alias>").post(find_cascade_combo_by_alias))
        .push(Router::with_path("cascade/combo/code/<code>").post(find_cascade_combo_by_code))
        .push(Router::with_path("cascade/combo/id/<dataId>").post(find_cascade_combo_by_id))
        .push(Router::with_path("translate/alias/<alias>").post(find_translate_by_alias))
        .push(Router::with_path("translate/code/<code>").post(find_translate_by_code))
        .push(Router::with_path("translate/id/<dataId>").post(find_translate_by_id))
        .push(
            Router::with_path("cascade/translate/alias/<alias>")
                .post(find_cascade_translate_by_alias),
        )
        .push(
            Router::with_path("cascade/translate/code/<code>").post(find_cascade_translate_by_code),
        )
        .push(Router::with_path("cascade/translate/id/<dataId>").post(find_cascade_translate_by_id))
        .push(Router::with_path("multi/combo/alias").post(find_multi_combo_by_alias))
        .push(Router::with_path("multi/combo/code").post(find_multi_combo_by_code))
        .push(Router::with_path("multi/combo/id").post(find_multi_combo_by_id))
        .push(
            Router::with_path("multi/cascade/combo/alias").post(find_multi_cascade_combo_by_alias),
        )
        .push(Router::with_path("multi/cascade/combo/code").post(find_multi_cascade_combo_by_code))
        .push(Router::with_path("multi/cascade/combo/id").post(find_multi_cascade_combo_by_id))
        .push(Router::with_path("multi/translate/alias").post(find_multi_translate_by_alias))
        .push(Router::with_path("multi/translate/code").post(find_multi_translate_by_code))
        .push(Router::with_path("multi/translate/id").post(find_multi_translate_by_id))
        .push(
            Router::with_path("multi/cascade/translate/alias")
                .post(find_multi_cascade_translate_by_alias),
        )
        .push(
            Router::with_path("multi/cascade/translate/code")
                .post(find_multi_cascade_translate_by_code),
        )
        .push(
            Router::with_path("multi/cascade/translate/id")
                .post(find_multi_cascade_translate_by_id),
        );

    dict_router
}
