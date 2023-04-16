use serde_json::Value;

use bmbp_orm_ins::{BmbpORM, BmbpOrmSQL, BmbpScriptSql};
use bmbp_types::vo::BaseOrmModel;
use bmbp_types::vo::QueryPageParam;
use bmbp_types::{BmbpMap, BmbpResp, BmbpValue, BmbpVec, PageInner, RespVo};
use bmbp_util::snake_to_camel;

use crate::organ::{BmbpRbacOrgan, OrganQueryParam};

pub struct OrganSql();

impl OrganSql {
    fn find_organ_sql(&self) -> BmbpScriptSql {
        let mut orm_sql = BmbpScriptSql::new();
        orm_sql.from("");
        orm_sql
    }
    fn insert_organ_sql(&self) -> BmbpScriptSql {
        let insert_script = BmbpScriptSql::new();
        insert_script
    }
    fn update_organ_sql() -> BmbpScriptSql {
        let update_script = BmbpScriptSql::new();
        update_script
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
}

pub struct OrganDao();

impl OrganDao {
    pub async fn find_organ_page(
        page_params: &mut OrganQueryParam,
    ) -> BmbpResp<PageInner<BmbpRbacOrgan>> {
        tracing::info!("组织机构数据服务-调用分页查询");
        let organ_query_sql = BmbpRbacOrgan::query_sql();
        let page_inner = BmbpORM
            .await
            .script_query_page(
                &organ_query_sql.to_sql_string(),
                &BmbpMap::new(),
                page_params.get_page_no().clone(),
                page_params.get_page_size().clone(),
            )
            .await?;
        let page_data_opt = page_inner.data();
        let mut new_page_inner: PageInner<BmbpRbacOrgan> = PageInner::new();
        match page_data_opt {
            None => {
                new_page_inner.set_page_no(page_inner.page_no());
                new_page_inner.set_page_size(page_inner.page_size());
                new_page_inner.set_total(page_inner.total())
            }
            Some(page_data) => {
                let vo_str = serde_json::to_string(page_data).unwrap();
                let organ_model: Vec<BmbpRbacOrgan> = serde_json::from_str(&vo_str).unwrap();
                new_page_inner.set_data(organ_model);
                new_page_inner.set_page_no(page_inner.page_no());
                new_page_inner.set_page_size(page_inner.page_size());
                new_page_inner.set_total(page_inner.total())
            }
        };
        Ok(new_page_inner)
    }
    pub async fn find_organ_list(params: &OrganQueryParam) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        tracing::info!("组织机构数据服务-调用列表查询");
        let orm_sql = BmbpRbacOrgan::query_sql();
        let vo_list = BmbpORM
            .await
            .script_query_list(&orm_sql.to_sql_string(), &BmbpMap::new())
            .await?;
        let v = match vo_list {
            None => None,
            Some(v) => {
                let json_str = serde_json::to_string(&v).unwrap();
                let model: Vec<BmbpRbacOrgan> = serde_json::from_str(&json_str).unwrap();
                Some(model)
            }
        };
        Ok(v)
    }
    pub async fn find_organ_info_by_rid(r_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        tracing::info!("调用组织数据库接口服务-查询组织详情");
        let mut script_sql = BmbpRbacOrgan::query_sql();
        script_sql.filter("r_id=#{rId}");
        let mut script_params = BmbpMap::new();
        script_params.insert("rId".to_string(), BmbpValue::from(r_id));
        let bmbp_value = BmbpORM
            .await
            .script_query_one(&script_sql.to_sql_string(), &script_params)
            .await?;
        let organ_model = match bmbp_value {
            None => None,
            Some(vo) => {
                let vo_str = serde_json::to_string(&vo).unwrap();
                let organ_model: BmbpRbacOrgan = serde_json::from_str(&vo_str).unwrap();
                Some(organ_model)
            }
        };
        Ok(organ_model)
    }
    pub async fn find_organ_info_by_organ_id(organ_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        tracing::info!("调用组织数据库接口服务-查询组织详情");
        Ok(None)
    }
    pub async fn insert_organ(params: &BmbpRbacOrgan) -> BmbpResp<usize> {
        let insert_sql = BmbpRbacOrgan::insert_sql();
        let vo = BmbpORM
            .await
            .script_insert(&insert_sql.to_sql_string(), &BmbpMap::new())
            .await?;
        Ok(vo)
    }
    pub async fn update_organ(params: &BmbpRbacOrgan) -> BmbpResp<usize> {
        let update_sql = OrganSql::update_organ_sql();
        let row_count = BmbpORM
            .await
            .script_update(&update_sql.to_sql_string(), &BmbpMap::new())
            .await?;
        Ok(row_count)
    }
    pub(crate) async fn update_organ_parent(params: &BmbpMap) -> BmbpResp<usize> {
        let update_organ_sql = OrganSql::update_organ_parent_sql();
        Ok(BmbpORM
            .await
            .script_update(&update_organ_sql, params)
            .await?)
    }
    pub(crate) async fn delete_organ(r_id: &String) -> BmbpResp<usize> {
        let delete_sql = BmbpRbacOrgan::delete_sql();
        let mut bmbp_params = BmbpMap::new();
        bmbp_params.insert("rId".to_string(), BmbpValue::from(r_id));
        tracing::info!("执行删除...");
        Ok(BmbpORM
            .await
            .script_delete(&delete_sql.to_sql_string(), &bmbp_params)
            .await?)
    }
}
