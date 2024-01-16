use bmbp_app_common::{BmbpResp, BmbpTreeModel, PageParams, PageVo};
use bmbp_app_curd::BmbpCurdScript;
use bmbp_orm_sql::SqlBuilder;
use crate::dict::dao::BmbpSettingDictDao;
use crate::dict::model::{BmbpSettingDictOrmTreeModel, DictQueryParams};

pub struct BmbpSettingDictService;

impl BmbpSettingDictService {
    pub async fn find_dict_tree(params: &DictQueryParams) -> BmbpResp<Vec<BmbpSettingDictOrmTreeModel>> {
        let query_sql = BmbpCurdScript::query::<BmbpSettingDictOrmTreeModel>();
        tracing::info!("query_sql: {}", query_sql.build());
        let dict_vec = BmbpSettingDictDao::query(&query_sql.build(), &params).await?;
        if dict_vec.is_none() {
            return Ok(vec![]);
        }
        let tree_node: Vec<BmbpSettingDictOrmTreeModel> = BmbpTreeModel::build_tree_without_spurious(dict_vec.unwrap());
        return Ok(tree_node);
    }
    pub async fn find_dict_page(params: &PageParams<DictQueryParams>) -> BmbpResp<PageVo<BmbpSettingDictOrmTreeModel>> {
        let query_sql = BmbpCurdScript::query::<BmbpSettingDictOrmTreeModel>();
        tracing::info!("query_sql: {}", query_sql.build());
        return Ok(PageVo::new());
    }
}