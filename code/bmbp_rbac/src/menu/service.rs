use crate::menu::dao::MenuDao;
use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};
use bmbp_base::{CurdDao, CurdService};
use bmbp_types::{BmbpResp, PageInner, PageReqVo, RespVo};

pub struct MenuService {
    dao: MenuDao,
}

impl MenuService {
    pub fn new() -> Self {
        MenuService {
            dao: MenuDao::new(),
        }
    }
}
