use async_trait::async_trait;
use bmbp_app_curd::CurdDao;
pub struct UserDao;
#[async_trait]
impl CurdDao for UserDao {}
impl UserDao {}
