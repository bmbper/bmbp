use salvo::{handler, Request, Response};

#[handler]
pub async fn find_user_tree(_req: &mut Request, _res: &mut Response) {}
