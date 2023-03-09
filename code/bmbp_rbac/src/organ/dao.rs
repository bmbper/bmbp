use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL};
use bmbp_types::vo::BaseOrmModel;
use bmbp_types::vo::QueryPageParam;
use bmbp_types::{BmbpResp, PageInner};
use serde_json::Value;

use crate::organ::vopo::{PageQueryParam, BMBP_RBAC_ORGAN};

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganSql();

impl OrganSql {
    pub fn find_organ_base_sql(query_params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::query();
        let organ_table_columns = BmbpOrganVo::orm_fields();
        for item in organ_table_columns {
            orm_sql.as_query_mut()?.select_column(item.clone());
        }
        orm_sql
            .as_query_mut()?
            .target_table(BMBP_RBAC_ORGAN.to_string());

        if !query_params.get_organ_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("organId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "organId".to_string(),
                Value::String(query_params.get_organ_id().to_string()),
            );
        }
        if !query_params.get_r_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("rId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "rId".to_string(),
                Value::String(query_params.get_r_id().to_string()),
            );
        }
        Ok(orm_sql)
    }

    pub fn find_organ_list_sql(query_params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = Self::find_organ_base_sql(query_params)?;
        if !query_params.get_organ_title().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_lk("organTitle".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "organTitle".to_string(),
                Value::String(query_params.get_organ_title().to_string()),
            );
        }
        if !query_params.get_organ_path().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_llk("organPath".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "organPath".to_string(),
                Value::String(query_params.get_organ_path().to_string()),
            );
        }

        if !query_params.get_parent_organ_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("parentOrganId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "parentOrganId".to_string(),
                Value::String(query_params.get_parent_organ_id().to_string()),
            );
        }

        Ok(orm_sql)
    }

    pub fn find_organ_info_sql(query_params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        Self::find_organ_base_sql(query_params)
    }

    pub fn insert_organ(params: &BmbpOrganVo) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::insert();
        let insert_sql = orm_sql.as_insert_mut()?;
        insert_sql.target_table(BMBP_RBAC_ORGAN.to_string());
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

    pub fn update_organ_parent_sql(params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::update();

        orm_sql
            .as_update_mut()?
            .target_table(BMBP_RBAC_ORGAN.to_string());

        orm_sql
            .as_update_mut()?
            .set_s_f("parentOrganId".to_string());
        orm_sql.get_mut_dynamic_params().add_k_param(
            "parentOrganId".to_string(),
            Value::String(params.get_parent_organ_id().to_string()),
        );

        if !params.get_r_id().is_empty() {
            orm_sql
                .as_update_mut()?
                .get_mut_filter()
                .s_f_eq("rId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "rId".to_string(),
                Value::String(params.get_organ_id().to_string()),
            );
        } else if !params.get_organ_id().is_empty() {
            orm_sql
                .as_update_mut()?
                .get_mut_filter()
                .s_f_eq("organId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "organId".to_string(),
                Value::String(params.get_organ_id().to_string()),
            );
        }
        Ok(orm_sql)
    }
    fn delete_organ_sql(params: &QueryParam) -> BmbpResp<BmbpOrmSQL> {
        let raw_r_id_vec = {
            if !params.get_r_id().is_empty() {
                let r_id_vec = params.get_r_id().split(",").collect();
                r_id_vec
            } else {
                vec![]
            }
        };
        let raw_organ_id_vec = {
            if !params.get_organ_id().is_empty() {
                let r_id_vec = params.get_organ_id().split(",").collect();
                r_id_vec
            } else {
                vec![]
            }
        };

        let mut orm_sql = BmbpOrmSQL::delete();
        orm_sql
            .as_delete_mut()?
            .target_table(BMBP_RBAC_ORGAN.to_string());

        if !raw_r_id_vec.is_empty() {
            if raw_r_id_vec.len() == 1 {
                orm_sql
                    .as_delete_mut()?
                    .get_mut_filter()
                    .s_f_eq("rId".to_string());
                orm_sql.get_mut_dynamic_params().add_k_param(
                    "rId".to_string(),
                    Value::String(raw_r_id_vec.get(0).unwrap().to_string()),
                );
            } else {
                orm_sql
                    .as_delete_mut()?
                    .get_mut_filter()
                    .s_f_in("rId".to_string());
                orm_sql
                    .get_mut_dynamic_params()
                    .add_k_param("rId".to_string(), Value::from(raw_r_id_vec));
            }
        } else if !raw_organ_id_vec.is_empty() {
            if raw_organ_id_vec.len() == 1 {
                orm_sql
                    .as_delete_mut()?
                    .get_mut_filter()
                    .s_f_eq("organId".to_string());
                orm_sql.get_mut_dynamic_params().add_k_param(
                    "organId".to_string(),
                    Value::String(raw_organ_id_vec.get(0).unwrap().to_string()),
                );
            } else {
                orm_sql
                    .as_delete_mut()?
                    .get_mut_filter()
                    .s_f_in("organId".to_string());
                orm_sql
                    .get_mut_dynamic_params()
                    .add_k_param("organId".to_string(), Value::from(raw_organ_id_vec));
            }
        }

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
    pub async fn find_organ_list(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        tracing::info!("组织机构数据服务-调用列表查询");
        let orm_sql = OrganSql::find_organ_list_sql(params)?;
        let vo_list = BmbpORM.await.find_list(orm_sql).await?;
        let vo_str = serde_json::to_string(&vo_list).unwrap();
        let organ_list: Vec<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        Ok(organ_list)
    }

    pub async fn find_organ_page(
        page_params: &mut PageQueryParam,
    ) -> BmbpResp<PageInner<BmbpOrganVo>> {
        tracing::info!("组织机构数据服务-调用分页查询");
        let mut raw_params = &QueryParam::default();
        if let Some(temp_params) = page_params.get_page_param() {
            raw_params = temp_params;
        }
        let orm_sql = OrganSql::find_organ_list_sql(raw_params)?;
        let page_no = if page_params.get_page_no() > 0 {
            page_params.get_page_no().clone()
        } else {
            1
        };

        let page_size = if page_params.get_page_size() > 0 {
            page_params.get_page_size().clone()
        } else {
            10
        };

        let page_inner = BmbpORM
            .await
            .find_page(orm_sql, &page_no, &page_size)
            .await?;
        let vo_str = serde_json::to_string(&page_inner).unwrap();
        let organ_page: PageInner<BmbpOrganVo> = serde_json::from_str(&vo_str).unwrap();
        Ok(organ_page)
    }

    pub async fn find_organ_info(params: &QueryParam) -> BmbpResp<Option<BmbpOrganVo>> {
        tracing::info!("调用组织数据库接口服务-查询组织详情");
        let orm_sql = OrganSql::find_organ_info_sql(params)?;
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

    pub(crate) async fn update_organ_parent(params: &QueryParam) -> BmbpResp<usize> {
        let update_organ_sql = OrganSql::update_organ_parent_sql(params)?;
        Ok(BmbpORM.await.update(update_organ_sql).await?)
    }

    pub(crate) async fn delete_organ(params: &QueryParam) -> BmbpResp<usize> {
        let delete_sql = OrganSql::delete_organ_sql(params)?;
        tracing::info!("执行删除...");
        Ok(BmbpORM.await.delete(delete_sql).await?)
    }
}
