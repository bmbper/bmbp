use std::collections::HashMap;
use tracing::info;
use bmbp_app_common::BmbpResp;
use bmbp_orm_ins::BmbpORM;
use crate::dict::model::{BmbpSettingDict, BmbpSettingDictOrmModel, BmbpSettingDictOrmTreeModel, DictQueryParams};

pub struct BmbpSettingDictDao;

impl BmbpSettingDictDao {
    pub(crate) async fn query(sql: &String, params: &&DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmTreeModel>>> {
        info!("query for dict:{}",sql);
        BmbpORM.await.generate_script_query_list::<BmbpSettingDictOrmTreeModel>(sql,&HashMap::new()).await
    }
}