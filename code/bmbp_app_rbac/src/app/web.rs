use crate::app::model::BmbpRbacApp;
use bmbp_app_common::{BmbpResp, PageVo};
use salvo::{handler, Request, Response};

#[handler]
pub async fn find_app_page(req: &Request, resp: &mut Response) -> BmbpResp<PageVo<BmbpRbacApp>> {
    Ok(PageVo::new())
}

#[handler]
pub async fn find_app_list(
    req: &Request,
    resp: &mut Response,
) -> BmbpResp<Option<Vec<BmbpRbacApp>>> {
    Ok(None)
}
#[handler]
pub async fn find_app_info(req: &Request, resp: &mut Response) -> BmbpResp<Option<BmbpRbacApp>> {
    Ok(None)
}
#[handler]
pub async fn save_app(req: &Request, resp: &mut Response) -> BmbpResp<usize> {
    Ok(0)
}
#[handler]
pub async fn update_app(req: &Request, resp: &mut Response) -> BmbpResp<usize> {
    Ok(0)
}
#[handler]
pub async fn remove_app(req: &Request, resp: &mut Response) -> BmbpResp<usize> {
    Ok(0)
}
#[handler]
pub async fn enable_app(req: &Request, resp: &mut Response) -> BmbpResp<usize> {
    Ok(0)
}
#[handler]
pub async fn disable_app(req: &Request, resp: &mut Response) -> BmbpResp<usize> {
    Ok(0)
}
