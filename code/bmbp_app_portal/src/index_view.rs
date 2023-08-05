use bmbp_app_common::EmbedStatic;
use salvo::handler;
use salvo::serve_static::EmbeddedFileExt;
use salvo::writing::Redirect;
use salvo::Request;
use salvo::Response;

#[handler]
pub async fn index_view(req: &mut Request, _res: &mut Response) -> Redirect {
    if let Some(token) = req.query::<String>("token") {
        //TODO 从会话中取SessionId 判断是否有状态
        return Redirect::other(format!("/portal.view?token={}", token).as_str());
    } else {
        return Redirect::other("/login.view");
    }
}

#[handler]
pub async fn favicon(req: &mut Request, res: &mut Response) {
    if let Some(file) = EmbedStatic::get("asserts/image/favicon.ico") {
        file.render(req, res);
    }
}
