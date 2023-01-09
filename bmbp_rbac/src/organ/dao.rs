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
    pub fn find_list_sql(params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::query();
        let query_sql = orm_sql.as_query_mut()?;
        let orm_column = BmbpOrganVo::orm_fields();
        for item in orm_column {
            query_sql.select_c_as_df(item.clone());
        }
        query_sql.from(BMBP_RBAC_ORGAN.to_string());
        if !params.get_organ_path().is_empty() {
            query_sql.s_f_rlk("organPath".to_string());
        }

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
        params.add_k_param(
            "organId".to_string(),
            Value::String(query_params.get_organ_id().to_string()),
        );
        Ok(bmbp_sql)
    }

    pub fn insert_organ(params: &BmbpOrganVo) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::insert();
        let insert_sql = orm_sql.as_insert_mut()?;
        insert_sql.insert_into(BMBP_RBAC_ORGAN.to_string());
        let organ_fields = BmbpOrganVo::orm_fields();
        for field in organ_fields.as_slice() {
            insert_sql.c_insert(field.clone());
        }
        Self::build_organ_params(&mut orm_sql, params);
        Ok(orm_sql)
    }

    pub fn update_organ(params: &BmbpOrganVo) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::update();
        Self::build_organ_params(&mut orm_sql, params);
        Ok(orm_sql)
    }

    pub fn build_organ_params(orm_sql: &mut BmbpOrmSQL, params: &BmbpOrganVo) {
        let organ_params = orm_sql.get_mut_dynamic_params();
        organ_params.add_k_param(
            "organId".to_string(),
            Value::String(params.get_organ_id().clone()),
        );
        organ_params.add_k_param(
            "parentOrganId".to_string(),
            Value::String(params.get_parent_organ_id().clone()),
        );
        organ_params.add_k_param(
            "organTitle".to_string(),
            Value::String(params.get_organ_title().clone()),
        );
        organ_params.add_k_param(
            "organPath".to_string(),
            Value::String(params.get_organ_path().clone()),
        );
        organ_params.add_k_param(
            "organDataId".to_string(),
            Value::String(params.get_organ_data_id().clone()),
        );
        organ_params.add_k_param(
            "organType".to_string(),
            Value::String(params.get_organ_type().to_string()),
        );
        organ_params.add_k_param("rId".to_string(), Value::String(params.get_r_id().clone()));
        organ_params.add_k_param(
            "rLevel".to_string(),
            Value::String(params.get_r_level().clone()),
        );
        organ_params.add_k_param(
            "rFlag".to_string(),
            Value::String(params.get_r_flag().clone()),
        );
        organ_params.add_k_param(
            "rCreateTime".to_string(),
            Value::String(params.get_r_create_time().clone()),
        );
        organ_params.add_k_param(
            "rCreateUser".to_string(),
            Value::String(params.get_r_create_user().clone()),
        );
        organ_params.add_k_param(
            "rUpdateTime".to_string(),
            Value::String(params.get_r_update_time().clone()),
        );
        organ_params.add_k_param(
            "rUpdateUser".to_string(),
            Value::String(params.get_r_update_user().clone()),
        );
        organ_params.add_k_param(
            "rOwnerOrg".to_string(),
            Value::String(params.get_r_owner_org().clone()),
        );
        organ_params.add_k_param(
            "rOwnerUser".to_string(),
            Value::String(params.get_r_owner_user().clone()),
        );
        organ_params.add_k_param(
            "rSign".to_string(),
            Value::String(params.get_r_sign().clone()),
        );
    }
}

pub struct OrganDao();

impl OrganDao {
    pub async fn find_grid_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let orm_sql = OrganSql::find_list_sql(params)?;
        let vo_list = BmbpORM.await.find_list(orm_sql).await?;
        let vo_str = serde_json::to_string(&vo_list).unwrap();
        let organ_list: Vec<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        Ok(organ_list)
    }

    pub async fn find_page_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let orm_sql = OrganSql::find_list_sql(params)?;
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
        let insert_sql = OrganSql::insert_organ(params)?;
        let row_count = BmbpORM.await.insert(insert_sql).await?;
        Ok(row_count)
    }
    pub async fn update_organ(params: &BmbpOrganVo) -> BmbpResp<usize> {
        let update_sql = OrganSql::update_organ(params)?;
        let row_count = BmbpORM.await.update(update_sql).await?;
        Ok(row_count)
    }
}
