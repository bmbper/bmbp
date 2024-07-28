use crate::vars::bean::BmbpVars;
use bmbp_app_orm::QueryWrapper;

pub struct VarsService {}
impl VarsService {
    pub fn new() -> Self {
        VarsService {}
    }
    pub fn save(vars: BmbpVars) {
        let query_wrapper = QueryWrapper::new_from();
    }
}
