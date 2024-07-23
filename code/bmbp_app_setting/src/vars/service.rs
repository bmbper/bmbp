use crate::vars::bean::BmbpVars;
use bmbp_rdbc_orm::QueryWrapper;

pub struct VarsService {}
impl VarsService {
    pub fn new() -> Self {
        VarsService {}
    }
    pub fn save(vars: BmbpVars) {
        let query_wrapper = QueryWrapper::new_from::<BmbpVars>();
    }
}
