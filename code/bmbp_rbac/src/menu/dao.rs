use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam};
use bmbp_base::CurdDao;
use bmbp_orm_ins::BmbpOrmSQL;
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BmbpResp, PageReqVo};
use serde::{Deserialize, Serialize};

pub struct MenuDao {}
impl MenuDao {
    pub fn new() -> Self {
        MenuDao {}
    }
}
