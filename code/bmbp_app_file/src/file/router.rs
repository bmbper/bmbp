use super::web::*;
use salvo::size_limiter::max_size;
use salvo::Router;

pub fn file_router() -> Router {
    Router::new()
        .push(
            Router::new()
                .hoop(max_size(1024 * 1024 * 256))
                .path("/upload")
                .post(upload_file),
        )
        .push(Router::with_path("upload/img").post(upload_img))
        .push(
            Router::with_path("download")
                .post(download_file)
                .get(download_file),
        )
        .push(
            Router::with_path("download/id/<file_id>")
                .post(download_file_by_id)
                .get(download_file_by_id),
        )
        .push(
            Router::with_path("download/url/<file_url>")
                .post(download_file_by_url)
                .get(download_file_by_url),
        )
        .push(Router::with_path("file/info").post(find_file_info))
        .push(
            Router::with_path("file/info/id/<file_id>")
                .post(find_file_info_by_id)
                .get(find_file_info_by_id),
        )
        .push(
            Router::with_path("file/info/url/<file_url>")
                .post(find_file_info_by_url)
                .get(find_file_info_by_url),
        )
        .push(Router::with_path("file/page").post(find_file_page))
        .push(Router::with_path("file/list").post(find_file_list))
}
