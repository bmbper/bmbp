use bmbp_types::BmbpResp;
use bmbp_util::TreeBuilder;

use crate::organ::dao::OrganDao;

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganService();

impl OrganService {
    pub async fn find_tree_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let rows = Self::find_grid_data(params);
        let tree_organ = TreeBuilder::build(rows);
        Ok(tree_organ)
    }

    pub async fn find_grid_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let rows = OrganDao::find_grid_data(params).await?;
        Ok(rows)
    }
}
