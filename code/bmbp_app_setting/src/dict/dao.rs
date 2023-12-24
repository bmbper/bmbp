use bmbp_app_common::BmbpResp;
use bmbp_orm_ins::BmbpORM;
use crate::dict::model::{BmbpSettingDict, BmbpSettingDictOrmTreeModel, DictQueryParams};

pub struct BmbpSettingDictDao;

impl BmbpSettingDictDao {
    pub(crate) async fn query(sql: &String, params: &&DictQueryParams) -> BmbpResp<Vec<BmbpSettingDictOrmTreeModel>>{
        return Ok(vec![]);
    }
}