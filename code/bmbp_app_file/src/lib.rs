use salvo::serve_static::StaticDir;
use salvo::Router;

mod file;
use file::file_router;
mod embed;
use embed::build_embed_router;

pub fn build_file_router() -> Router {
    Router::new()
        .push(Router::with_path("/bmbp/file").push(file_router()))
        .push(
            Router::with_path("/static/<**path>").get(StaticDir::new(["webapp/lib"]).listing(true)),
        )
        .push(build_embed_router())
}
