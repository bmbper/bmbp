use crate::menu::vopo::{BmbpMenuVo, MenuQueryParam, BMBP_RBAC_MENU};
use bmbp_base::{CurdDao, CurdService};
use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL};
use bmbp_types::vo::{BaseOrmVoPo, QueryPageParam};
use bmbp_types::{BmbpError, BmbpResp, PageInner, PageReqVo};
use serde::Serialize;
use serde_json::Value;

pub struct MenuDao {}

impl MenuDao {
    pub(crate) fn orm_query_sql(params: &MenuQueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::query();
        for column in BmbpMenuVo::vo_fields().as_slice() {
            orm_sql.as_query_mut()?.select_column(column.to_string());
        }
        orm_sql
            .as_query_mut()?
            .target_table(BMBP_RBAC_MENU.to_string());

        // filter
        if !params.get_r_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("rId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "rId".to_string(),
                Value::String(params.get_r_id().to_string()),
            );
        } else {
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
        }

        Ok(orm_sql)
    }

    pub(crate) fn orm_delete_sql(params: &MenuQueryParam) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::delete();
        orm_sql
            .as_delete_mut()?
            .target_table(BMBP_RBAC_MENU.to_string());

        if !params.get_r_id().is_empty() {
            orm_sql
                .as_query_mut()?
                .get_mut_filter()
                .s_f_eq("rId".to_string());
            orm_sql.get_mut_dynamic_params().add_k_param(
                "rId".to_string(),
                Value::String(params.get_r_id().to_string()),
            );
        } else {
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
        }
        Ok(orm_sql)
    }

    pub(crate) fn orm_insert_sql(po: &BmbpMenuVo) -> BmbpResp<BmbpOrmSQL> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    pub(crate) fn orm_update_sql(params: &MenuQueryParam, po: &BmbpMenuVo) -> BmbpResp<BmbpOrmSQL> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

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
