use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;

use super::model::BmbpRbacOrgan;

pub struct OrganDao();
#[allow(dead_code)]
impl OrganDao {
    pub(crate) async fn find_organ_tree(
        sql_scirpt: &String,
        params: &BmbpHashMap,
    ) -> BmbpResp<Vec<BmbpRbacOrgan>> {
        Err(BmbpError::orm("数据库查询未实现".to_string()))
    }
}
