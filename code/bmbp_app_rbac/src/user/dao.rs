use async_trait::async_trait;
use bmbp_app_curd::CurdDaoTrait;
pub struct UserDao;
#[async_trait]
impl CurdDaoTrait for UserDao {}
impl UserDao {}
