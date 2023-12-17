use bmbp_app_common::{BmbpResp, BmbpTreeModel};
use bmbp_app_curd::BmbpCurdScript;
use crate::dict::model::{BmbpSettingDictOrmTreeModel, DictQueryParams};

pub struct BmbpSettingDictService;

impl BmbpSettingDictService {
    pub async fn find_dict_tree(params: &DictQueryParams) -> BmbpResp<Vec<BmbpSettingDictOrmTreeModel>> {
        let dict_vec = vec![];
        let tree_node: Vec<BmbpSettingDictOrmTreeModel> = BmbpTreeModel::build_tree_without_spurious(dict_vec);
        return Ok(tree_node);
    }
}