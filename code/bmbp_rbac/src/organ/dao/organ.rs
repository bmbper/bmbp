use serde_json::Value;

use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL, BmbpScriptSql};
use bmbp_types::vo::BaseOrmModel;
use bmbp_types::vo::QueryPageParam;
use bmbp_types::{BmbpMap, BmbpResp, BmbpValue, BmbpVec, PageInner, RespVo};
use bmbp_util::{snake_to_camel, BmbpValueUtil};

use crate::organ::model::{PageQueryParam, BMBP_RBAC_ORGAN};
use crate::organ::{BmbpOrganModel, QueryParam};

pub struct OrganSql();

impl OrganSql {
    fn find_organ_base_sql() -> BmbpScriptSql {
        let mut orm_sql = BmbpScriptSql::new();
        orm_sql.from(BMBP_RBAC_ORGAN);
        for item in BmbpOrganModel::orm_fields().as_slice() {
            orm_sql.select(&format!(
                "{} as \"{}\"",
                item,
                snake_to_camel(item.to_string())
            ));
        }
        orm_sql
    }
    fn find_organ_list_sql(query_params: &QueryParam) -> BmbpScriptSql {
        Self::find_organ_base_sql()
    }
    fn find_organ_info_sql(query_params: &QueryParam) -> BmbpScriptSql {
        Self::find_organ_base_sql()
    }
    fn insert_organ(params: &BmbpOrganModel) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::insert();
        let insert_sql = orm_sql.as_insert_mut()?;
        insert_sql.target_table(BMBP_RBAC_ORGAN.to_string());
        let organ_fields = BmbpOrganModel::orm_fields();
        for field in organ_fields.as_slice() {
            insert_sql.c_insert(field.clone());
        }
        Self::build_organ_params(&mut orm_sql, params);
        Ok(orm_sql)
    }
    fn update_organ(params: &BmbpOrganModel) -> BmbpResp<BmbpOrmSQL> {
        let mut orm_sql = BmbpOrmSQL::update();
        Self::build_organ_params(&mut orm_sql, params);
        Ok(orm_sql)
    }
    fn update_organ_parent_sql() -> String {
        let mut sql = BmbpScriptSql::new();
        sql.update("bmbp_rbac_organ");
        sql.set_value(
            "organ_path",
            "concat(#{targetPath}, RIGHT(organ_path, LENGTH(organ_path)-LENGTH(#{currentPath})))",
        );
        sql.filter("organ_path like concat(#{currentPath},'%')");
        sql.to_sql_string()
    }
    fn delete_organ_sql() -> BmbpScriptSql {
        let mut delete_sql = BmbpScriptSql::new();
        delete_sql.from(BMBP_RBAC_ORGAN);
        delete_sql.filter("r_id=#{rId}");
        delete_sql
    }
    pub fn build_organ_params(orm_sql: &mut BmbpOrmSQL, params: &BmbpOrganModel) {
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
    pub async fn find_organ_list(params: &QueryParam) -> BmbpResp<Option<Vec<BmbpOrganModel>>> {
        tracing::info!("组织机构数据服务-调用列表查询");
        let orm_sql = OrganSql::find_organ_base_sql();
        let vo_list = BmbpORM
            .await
            .script_query_list(&orm_sql.to_sql_string(), &BmbpMap::new())
            .await?;
        let v = match vo_list {
            None => None,
            Some(v) => {
                let json_str = serde_json::to_string(&v).unwrap();
                let model: Vec<BmbpOrganModel> = serde_json::from_str(&json_str).unwrap();
                Some(model)
            }
        };
        Ok(v)
    }
    pub async fn find_organ_page(
        page_params: &mut PageQueryParam,
    ) -> BmbpResp<PageInner<BmbpOrganModel>> {
        tracing::info!("组织机构数据服务-调用分页查询");
        let orm_sql = OrganSql::find_organ_base_sql();
        let page_inner = BmbpORM
            .await
            .script_query_page(
                &orm_sql.to_sql_string(),
                &BmbpMap::new(),
                page_params.get_page_no(),
                page_params.get_page_size(),
            )
            .await?;
        let page_data_opt = page_inner.data();
        let mut new_page_inner: PageInner<BmbpOrganModel> = PageInner::new();
        match page_data_opt {
            None => {
                new_page_inner.set_page_no(page_inner.page_no());
                new_page_inner.set_page_size(page_inner.page_size());
                new_page_inner.set_total(page_inner.total())
            }
            Some(page_data) => {
                let vo_str = serde_json::to_string(page_data).unwrap();
                let organ_model: Vec<BmbpOrganModel> = serde_json::from_str(&vo_str).unwrap();
                new_page_inner.set_data(organ_model);
                new_page_inner.set_page_no(page_inner.page_no());
                new_page_inner.set_page_size(page_inner.page_size());
                new_page_inner.set_total(page_inner.total())
            }
        };
        Ok(new_page_inner)
    }
    pub async fn find_organ_info_by_rid(r_id: &String) -> BmbpResp<Option<BmbpOrganModel>> {
        tracing::info!("调用组织数据库接口服务-查询组织详情");
        let mut script_sql = OrganSql::find_organ_base_sql();
        script_sql.filter("r_id=#{rId}");
        let mut script_params = BmbpMap::new();
        script_params.insert("rId".to_string(), BmbpValue::from_string_ref(r_id));
        let bmbp_value = BmbpORM
            .await
            .script_query_one(&script_sql.to_sql_string(), &script_params)
            .await?;
        let organ_model = match bmbp_value {
            None => None,
            Some(vo) => {
                let vo_str = serde_json::to_string(&vo).unwrap();
                let organ_model: BmbpOrganModel = serde_json::from_str(&vo_str).unwrap();
                Some(organ_model)
            }
        };
        Ok(organ_model)
    }
    pub async fn find_organ_info_by_organ_id(
        organ_id: &String,
    ) -> BmbpResp<Option<BmbpOrganModel>> {
        tracing::info!("调用组织数据库接口服务-查询组织详情");
        Ok(None)
    }
    pub async fn insert_organ(params: &BmbpOrganModel) -> BmbpResp<usize> {
        let insert_sql = OrganSql::insert_organ(params)?;
        let row_count = BmbpORM.await.insert(insert_sql).await?;
        Ok(row_count)
    }
    pub async fn update_organ(params: &BmbpOrganModel) -> BmbpResp<usize> {
        let update_sql = OrganSql::update_organ(params)?;
        let row_count = BmbpORM.await.update(update_sql).await?;
        Ok(row_count)
    }
    pub(crate) async fn update_organ_parent(params: &BmbpMap) -> BmbpResp<usize> {
        let update_organ_sql = OrganSql::update_organ_parent_sql();
        Ok(BmbpORM
            .await
            .script_update(&update_organ_sql, params)
            .await?)
    }
    pub(crate) async fn delete_organ(params: &QueryParam) -> BmbpResp<usize> {
        let delete_sql = OrganSql::delete_organ_sql();
        tracing::info!("执行删除...");
        Ok(BmbpORM
            .await
            .script_delete(&delete_sql.to_sql_string(), &BmbpMap::new())
            .await?)
    }
}
