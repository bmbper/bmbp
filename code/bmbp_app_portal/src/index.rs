use salvo::handler;
use salvo::Request;
use salvo::Response;

#[handler]
pub fn index_api(_req: &mut Request, res: &mut Response) {
    res.render("欢迎使用bmbp应用模块，本项目仅提供后台接口，前端项目请移步bmbp_ui")
}
