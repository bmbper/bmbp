use salvo::handler;
use salvo::Request;
use salvo::Response;

#[handler]
pub fn upload_file(_req: &mut Request, _res: &mut Response) {}

/// 上传图片
#[handler]
pub async fn upload_img(_req: &mut Request, _res: &mut Response) {}

#[handler]
pub async fn download_file(_req: &mut Request, _res: &mut Response) {}

/// 下载指定ID的附件
#[handler]
pub async fn download_file_by_id(_req: &mut Request, _res: &mut Response) {}

/// 下载指定路径的附件
#[handler]
pub async fn download_file_by_url(_req: &mut Request, _res: &mut Response) {}

/// 查询指定ID的附件信息
#[handler]
pub async fn find_file_info(_req: &mut Request, _res: &mut Response) {}

/// 查询指定ID的附件信息
#[handler]
pub async fn find_file_info_by_id(_req: &mut Request, _res: &mut Response) {}

/// 查询指定路径的附件信息
#[handler]
pub async fn find_file_info_by_url(_req: &mut Request, _res: &mut Response) {}

/// 查询附件分页列表
#[handler]
pub async fn find_file_page(_req: &mut Request, _res: &mut Response) {}

/// 查询附件列表
#[handler]
pub async fn find_file_list(_req: &mut Request, _res: &mut Response) {}
