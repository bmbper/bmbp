use salvo::handler;
use salvo::Request;
use salvo::Response;
#[handler]
pub async fn login(_req: &mut Request, res: &mut Response) {
    res.render("login")
}
#[handler]
pub async fn login_out(_req: &mut Request, res: &mut Response) {
    res.render("logout")
}
