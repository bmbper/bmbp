use axum::extract::Query;
use serde_json::Value;

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
            query_sql.select(item.clone());
        }
        query_sql.from("bmbp_rbac_organ".to_string());
        Ok(orm_sql)
    }

    pub fn find_organ_info_by_organ_id(query_params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut bmbp_sql = BmbpOrmSQL::query();
        let query_sql = bmbp_sql.as_query_mut()?;
        let orm_column = BmbpOrganVo::orm_fields();
        for item in orm_column {
            query_sql.select(item.clone());
        }
        query_sql.from("bmbp_rbac_organ".to_string());

        // filter
        query_sql.s_f_eq("organId".to_string());
        query_sql.s_c_eq("organId".to_string());
        query_sql.s_f_eq_as("organId".to_string(), "name".to_string());
        query_sql.s_c_eq_as("organ_id".to_string(), "name".to_string());
        query_sql.r_f_eq_string("organ_id".to_string(), "name".to_string());
        query_sql.r_f_eq_isize("organ_id".to_string(), 0);
        query_sql.r_c_eq_string("organ_id".to_string(), "name".to_string());
        query_sql.r_c_eq_f64("organ_id".to_string(), 0.0);

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
        let orm_sql = OrganSql::find_organ_info_by_organ_id(params)?;
        if let Some(vo) = BmbpORM.await.find_one(orm_sql).await? {
            let vo_str = serde_json::to_string(&vo).unwrap();
            let organ: BmbpOrganVo = serde_json::from_str(&vo_str).unwrap();
            Ok(Some(organ))
        } else {
            Ok(None)
        }
    }
}
