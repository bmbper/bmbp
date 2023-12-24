use bmbp_app_common::{BmbpResp, BmbpTreeModel};
use bmbp_app_curd::BmbpCurdScript;
use bmbp_orm_sql::SqlBuilder;
use crate::dict::dao::BmbpSettingDictDao;
use crate::dict::model::{BmbpSettingDictOrmTreeModel, DictQueryParams};

pub struct BmbpSettingDictService;

impl BmbpSettingDictService {
    pub async fn find_dict_tree(params: &DictQueryParams) -> BmbpResp<Vec<BmbpSettingDictOrmTreeModel>> {
        let query_sql = BmbpCurdScript::query::<BmbpSettingDictOrmTreeModel>();
        tracing::info!("query_sql: {}", query_sql.build());
        let dict_vec = BmbpSettingDictDao::query(&query_sql.build(),&params).await?;
        let tree_node: Vec<BmbpSettingDictOrmTreeModel> = BmbpTreeModel::build_tree_without_spurious(dict_vec);
        return Ok(tree_node);
    }
}