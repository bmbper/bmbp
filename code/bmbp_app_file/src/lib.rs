use salvo::Router;

mod file;
mod tmpl;
mod view;

use crate::view::file_view;
use file::file_router;

pub fn build_file_router() -> Router {
    Router::new().push(
        Router::with_path("bmbp/file")
            .push(Router::with_path("manage").get(file_view))
            .push(file_router()),
    )
}
