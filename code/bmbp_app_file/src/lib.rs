use bmbp_app_common::EmbedStatic;
use salvo::{
    serve_static::{static_embed, StaticDir},
    Router,
};

mod file;
use file::file_router;

pub fn build_file_router() -> Router {
    Router::new()
        .push(Router::with_path("/bmbp/file").push(file_router()))
        //.push(Router::with_path("/static/<**path>").get(StaticDir::new(vec!["webapp/static"])))
        .push(Router::with_path("/static/<**path>").get(static_embed::<EmbedStatic>()))
}
