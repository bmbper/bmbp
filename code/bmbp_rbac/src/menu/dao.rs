use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam, BMBP_RBAC_MENU};
use bmbp_base::{CurdDao, CurdService};
use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL};
use bmbp_types::vo::{BaseOrmVoPo, QueryPageParam};
use bmbp_types::{BmbpError, BmbpResp, PageInner, PageReqVo};
use serde::Serialize;
use serde_json::Value;

pub struct MenuDao {}

impl MenuDao {
    fn build_query_params_app_id(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if !params.get_app_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("appId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "appId".to_string(),
                Value::String(params.get_app_id().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_menu_id(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if !params.get_menu_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("menuId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "menuId".to_string(),
                Value::String(params.get_menu_id().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_menu_parent_id(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if !params.get_menu_parent_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("menuParentId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "menuParentId".to_string(),
                Value::String(params.get_menu_parent_id().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_menu_title(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if !params.get_menu_title().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_lk("menuTitle".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "menuTitle".to_string(),
                Value::String(params.get_menu_title().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_menu_path(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if !params.get_menu_path().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_llk("menuPath".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "menuPath".to_string(),
                Value::String(params.get_menu_path().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_menu_route_type(
        orm_sql: &mut BmbpOrmSQL,
        params: &MenuQueryParam,
    ) -> BmbpResp<()> {
        if params.get_menu_route_type().is_some() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("menuRouteType".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "menuRouteType".to_string(),
                Value::String(params.get_menu_route_type().unwrap().to_string()),
            );
        }
        Ok(())
    }
    fn build_query_params_r_id(orm_sql: &mut BmbpOrmSQL, params: &MenuQueryParam) -> BmbpResp<()> {
        orm_sql
            .as_query_mut()?
            .get_mut_filter()
            .s_f_eq("rId".to_string());
        orm_sql.get_mut_dynamic_params().add_k_param(
            "rId".to_string(),
            Value::String(params.get_r_id().to_string()),
        );
        Ok(())
    }
}
impl MenuDao {
    pub(crate) fn orm_query_sql(params: &MenuQueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::query();
        for column in BmbpMenuVo::orm_fields().as_slice() {
            orm_sql.as_query_mut()?.select_column(column.to_string());
        }
        orm_sql
            .as_query_mut()?
            .target_table(BMBP_RBAC_MENU.to_string());

        // filter
        if !params.get_r_id().is_empty() {
            Self::build_query_params_r_id(&mut orm_sql, params)?;
        } else {
            Self::build_query_params_app_id(&mut orm_sql, params)?;
            Self::build_query_params_menu_id(&mut orm_sql, params)?;
            Self::build_query_params_menu_title(&mut orm_sql, params)?;
            Self::build_query_params_menu_parent_id(&mut orm_sql, params)?;
            Self::build_query_params_menu_path(&mut orm_sql, params)?;
            Self::build_query_params_menu_route_type(&mut orm_sql, params)?;
        }

        // 排序
        orm_sql
            .as_query_mut()?
            .order_field_asc("menuOrder".to_string())
            .order_field_asc("menuPath".to_string());

        Ok(orm_sql)
    }

    pub(crate) fn orm_delete_sql(params: &MenuQueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::delete();
        orm_sql
            .as_delete_mut()?
            .target_table(BMBP_RBAC_MENU.to_string());

        if !params.get_r_id().is_empty() {
            Self::build_query_params_r_id(&mut orm_sql, params)?;
        } else {
            Self::build_query_params_app_id(&mut orm_sql, params)?;
            Self::build_query_params_menu_id(&mut orm_sql, params)?;
            Self::build_query_params_menu_parent_id(&mut orm_sql, params)?;
        }
        Ok(orm_sql)
    }

    pub(crate) fn orm_insert_sql(po: &BmbpMenuVo) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::insert();
        orm_sql
            .as_insert_mut()?
            .target_table(BMBP_RBAC_MENU.to_string());
        for field in BmbpMenuVo::orm_fields().as_slice() {
            orm_sql.as_insert_mut()?.c_insert(field.to_string());
        }

        let mut orm_params = orm_sql.get_mut_dynamic_params();
        orm_params.add_k_param("rId".to_string(), Value::String(po.get_r_id().clone()));
        orm_params.add_k_param("rFlag".to_string(), Value::String(po.get_r_flag().clone()));
        orm_params.add_k_param(
            "rLevel".to_string(),
            Value::String(po.get_r_level().clone()),
        );
        orm_params.add_k_param(
            "rCreateTime".to_string(),
            Value::String(po.get_r_create_time().clone()),
        );
        orm_params.add_k_param(
            "rCreateUser".to_string(),
            Value::String(po.get_r_create_user().clone()),
        );
        orm_params.add_k_param(
            "rUpdateTime".to_string(),
            Value::String(po.get_r_update_time().clone()),
        );
        orm_params.add_k_param(
            "rUpdateUser".to_string(),
            Value::String(po.get_r_update_user().clone()),
        );
        orm_params.add_k_param(
            "rOwnerOrg".to_string(),
            Value::String(po.get_r_owner_org().clone()),
        );
        orm_params.add_k_param(
            "rOwnerUser".to_string(),
            Value::String(po.get_r_owner_user().clone()),
        );
        orm_params.add_k_param("rSign".to_string(), Value::String(po.get_r_sign().clone()));

        orm_params.add_k_param(
            "menuId".to_string(),
            Value::String(po.get_menu_id().clone()),
        );
        orm_params.add_k_param(
            "parentMenuId".to_string(),
            Value::String(po.get_parent_menu_id().clone()),
        );
        orm_params.add_k_param(
            "menuTitle".to_string(),
            Value::String(po.get_menu_title().clone()),
        );
        orm_params.add_k_param(
            "menuPath".to_string(),
            Value::String(po.get_menu_path().clone()),
        );
        orm_params.add_k_param(
            "menuType".to_string(),
            Value::String(po.get_menu_type().to_string()),
        );
        if let Some(route_type) = po.get_menu_route_type() {
            orm_params.add_k_param(
                "menuRouteType".to_string(),
                Value::String(route_type.to_string()),
            );
        }

        orm_params.add_k_param(
            "menuRoute".to_string(),
            Value::String(po.get_menu_route().clone()),
        );
        orm_params.add_k_param("appId".to_string(), Value::String(po.get_app_id().clone()));
        Ok(orm_sql)
    }

    pub(crate) fn orm_update_sql(params: &MenuQueryParam, po: &BmbpMenuVo) -> BmbpResp<BmbpOrmSQL> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
}

impl MenuDao {
    pub(crate) async fn find_page(
        params: &PageReqVo<MenuQueryParam>,
    ) -> BmbpResp<PageInner<BmbpMenuVo>> {
        let mut mqp = &MenuQueryParam::default();
        if params.get_page_param().is_some() {
            mqp = params.get_page_param().unwrap();
        }
        let query_sql = Self::orm_query_sql(mqp)?;
        let vo_vec = BmbpORM
            .await
            .find_page(query_sql, &params.get_page_no(), &params.get_page_size())
            .await?;
        let vo_vec_json_string = serde_json::to_string(&vo_vec).unwrap();
        tracing::info!("po:{}", vo_vec_json_string);
        let vo_vec_vo: PageInner<BmbpMenuVo> = serde_json::from_str(&vo_vec_json_string).unwrap();
        Ok(vo_vec_vo)
    }

    pub(crate) async fn find_list(params: &MenuQueryParam) -> BmbpResp<Vec<BmbpMenuVo>> {
        let query_sql = Self::orm_query_sql(params)?;
        let vo_vec = BmbpORM.await.find_list(query_sql).await?;
        let vo_vec_json_string = serde_json::to_string(&vo_vec).unwrap();
        let vo_vec_vo: Vec<BmbpMenuVo> = serde_json::from_str(&vo_vec_json_string).unwrap();
        Ok(vo_vec_vo)
    }

    pub(crate) async fn find_one(params: &MenuQueryParam) -> BmbpResp<Option<BmbpMenuVo>> {
        let query_sql = Self::orm_query_sql(params)?;
        let vo_vec = BmbpORM.await.find_one(query_sql).await?;
        match vo_vec {
            Some(vo) => {
                let vo_vec_json_string = serde_json::to_string(&vo).unwrap();
                let vo_vec_vo: BmbpMenuVo = serde_json::from_str(&vo_vec_json_string).unwrap();
                Ok(Some(vo_vec_vo))
            }
            None => Ok(None),
        }
    }

    pub(crate) async fn delete(params: &MenuQueryParam) -> BmbpResp<usize> {
        let delete_sql = Self::orm_delete_sql(params)?;
        let row_count = BmbpORM.await.delete(delete_sql).await?;
        Ok(row_count)
    }

    pub(crate) async fn insert(po: &BmbpMenuVo) -> BmbpResp<usize> {
        let insert_sql = Self::orm_insert_sql(po)?;
        let row_count = BmbpORM.await.insert(insert_sql).await?;
        Ok(row_count)
    }

    pub(crate) async fn update(params: &MenuQueryParam, po: &BmbpMenuVo) -> BmbpResp<usize> {
        let update_sql = Self::orm_update_sql(params, po)?;
        let row_count = BmbpORM.await.insert(update_sql).await?;
        Ok(row_count)
    }
}
