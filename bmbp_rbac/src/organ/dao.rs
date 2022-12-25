use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL};
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::BmbpResp;
use bmbp_util::TreeBuilder;

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganSql();

impl OrganSql {
    pub fn list_sql(prams: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut bmbp_sql = BmbpOrmSQL::new();
        let query_sql = bmbp_sql.query()?;
        let orm_column = BmbpOrganVo::orm_fields();
        for item in orm_column {
            query_sql.select(item.clone());
        }
        query_sql.from("bmbp_rbac_organ".to_string());
        Ok(bmbp_sql)
    }
}

pub struct OrganDao();

impl OrganDao {
    pub async fn find_grid_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let orm_sql = OrganSql::list_sql(params)?;
        let vo_list = BmbpORM.await.find_list(orm_sql).await?;
        let vo_str = serde_json::to_string(&vo_list).unwrap();
        let organ_list: Vec<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        Ok(tree_organ)
    }

    pub async fn find_page_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let orm_sql = OrganSql::list_sql(params)?;
        let vo_list = BmbpORM.await.find_list(orm_sql).await?;
        let vo_str = serde_json::to_string(&vo_list).unwrap();
        let organ_list: Vec<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        let tree_organ = TreeBuilder::build(organ_list);
        Ok(tree_organ)
    }
}
