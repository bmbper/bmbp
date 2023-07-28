use bmbp_app_common::EmbedStatic;
use salvo::handler;
use salvo::serve_static::EmbeddedFileExt;
use salvo::Request;
use salvo::Response;

#[handler]
pub async fn index_view(_req: &mut Request, res: &mut Response) {
    res.render("wait redirect")
}

#[handler]
pub async fn favicon(req: &mut Request, res: &mut Response) {
    if let Some(file) = EmbedStatic::get("asserts/image/favicon.ico") {
        file.render(req, res);
    }
}
