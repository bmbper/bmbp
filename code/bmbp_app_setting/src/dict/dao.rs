use bmbp_app_common::BmbpResp;
use crate::dict::model::{BmbpSettingDictOrmModel,  DictQueryParams};

pub struct BmbpSettingDictDao;

impl BmbpSettingDictDao {
    pub(crate) async fn query(sql: &String, params: &&DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
       Ok(None)
    }
}