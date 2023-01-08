use axum::extract::Query;
use serde_json::Value;

use crate::organ::vopo::BMBP_RBAC_ORGAN;
use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL};
use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::BmbpResp;
use bmbp_util::TreeBuilder;

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganSql();

impl OrganSql {
    pub fn list_sql(prams: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::query();
        let query_sql = orm_sql.as_query_mut()?;
        let orm_column = BmbpOrganVo::orm_fields();
        for item in orm_column {
            query_sql.select_c_as_df(item.clone());
        }
        query_sql.from(BMBP_RBAC_ORGAN.to_string());
        Ok(orm_sql)
    }

    pub fn find_one_organ_by_organ_id_sql(query_params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut bmbp_sql = BmbpOrmSQL::query();

        let query_sql = bmbp_sql.as_query_mut()?;

        // append select field
        let orm_column = BmbpOrganVo::orm_fields();
        for item in orm_column {
            query_sql.select_c_as_df(item.clone());
        }
        // append table
        query_sql.from(BMBP_RBAC_ORGAN.to_string());
        // append filter
        query_sql.s_f_eq("organId".to_string());

        // append params
        let params = bmbp_sql.get_mut_dynamic_params();
        params.add_param(
            "organId".to_string(),
            Value::String(query_params.get_organ_id().to_string()),
        );
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
        Ok(organ_list)
    }

    pub async fn find_page_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let orm_sql = OrganSql::list_sql(params)?;
        let vo_list = BmbpORM.await.find_list(orm_sql).await?;
        let vo_str = serde_json::to_string(&vo_list).unwrap();
        let organ_list: Vec<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        let tree_organ = TreeBuilder::build(organ_list);
        Ok(tree_organ)
    }

    pub async fn find_one_by_organ_id(params: &QueryParam) -> BmbpResp<Option<BmbpOrganVo>> {
        let orm_sql = OrganSql::find_one_organ_by_organ_id_sql(params)?;
        if let Some(vo) = BmbpORM.await.find_one(orm_sql).await? {
            let vo_str = serde_json::to_string(&vo).unwrap();
            let organ: BmbpOrganVo = serde_json::from_str(&vo_str).unwrap();
            Ok(Some(organ))
        } else {
            Ok(None)
        }
    }
    pub async fn insert_organ(params: &BmbpOrganVo) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn update_organ(params: &BmbpOrganVo) -> BmbpResp<()> {
        Ok(())
    }
}
