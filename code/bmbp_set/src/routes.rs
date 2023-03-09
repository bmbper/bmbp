use crate::{dict, vars};
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn build_setting_router() -> Router {
    Router::new().nest(
        "v1/setting",
        Router::new()
            .nest(
                "/dict",
                Router::new()
                    .route("/tree", get(dict::find_tree))
                    .route("/page", get(dict::find_list))
                    .route("/list", get(dict::find_list))
                    .route("/info/id/:r_id", get(dict::find_info))
                    .route("/info/code/:dict_code", get(dict::find_info))
                    .route("/info/key/:dict_key", get(dict::find_info_by_key))
                    .route("/delete/id", get(dict::delete_by_ids))
                    .route("/delete/id/:r_id", get(dict::delete_by_id))
                    .route("/delete/key", get(dict::delete_by_ids))
                    .route("/delete/key/:dict_key", get(dict::delete_by_ids))
                    .route("/save", post(dict::save))
                    .route("/combo", get(dict::find_combo_by_keys))
                    .route("/combo/:dict_key", get(dict::find_combo_by_key))
                    .route("/combo/cascade", get(dict::find_cascade_combo_by_keys))
                    .route(
                        "/combo/cascade/:dict_key",
                        get(dict::find_cascade_combo_by_key),
                    ),
            )
            .nest(
                "/vars",
                Router::new()
                    .route("/list", get(vars::find_list))
                    .route("/info/id/:r_id", get(dict::find_info))
                    .route("/info/name/dict_name", get(dict::find_info_by_key))
                    .route("/save", post(dict::save))
                    .route("delete/id/:r_id", get(dict::delete_by_id)),
            ),
    )
}
