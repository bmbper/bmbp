use salvo::{handler, Request, Response};

#[handler]
pub async fn find_app(_req: &mut Request, _res: &mut Response) {}
#[handler]
pub async fn find_app_menu(_req: &mut Request, _res: &mut Response) {}
