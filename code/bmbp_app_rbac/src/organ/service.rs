use bmbp_app_common::{BmbpError, BmbpHashMap, BmbpResp, BmbpValue, PageParams, PageVo};
use bmbp_app_utils::TreeBuilder;
use bmbp_orm_ins::BmbpScriptSql;

use crate::organ::model::{BmbpRbacOrgan, OrganQueryParam};

use super::{dao::OrganDao, script::OrganScript};

/// 服务声明
pub struct OrganService();

/// CURD 逻辑
#[allow(dead_code)]
#[allow(unused)]
impl OrganService {
    /// 查询组织树
    pub async fn find_organ_tree(params: &OrganQueryParam) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        if let Some(record_id) = params.get_record_id() {
            return Self::find_organ_tree_start_with_id(record_id).await;
        }
        if let Some(organ_parent_code) = params.get_organ_parent_code() {
            return Self::find_organ_tree_start_with_code(organ_parent_code).await;
        }
        let query_script_sql = OrganScript::query_script();
        let organ_list_op =
            OrganDao::find_organ_tree(&query_script_sql.to_script(), &BmbpHashMap::new()).await?;

        match organ_list_op {
            Some(organ_list) => {
                let organ_tree = TreeBuilder::build::<BmbpRbacOrgan>(organ_list);
                Ok(Some(organ_tree))
            }
            None => Ok(None),
        }
    }
    pub async fn find_organ_tree_start_with_id(
        id: &String,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let organ_op = Self::find_organ_by_id(id).await?;
        Self::find_organ_tree_start_with_organ(organ_op).await
    }
    pub async fn find_organ_tree_start_with_code(
        code: &String,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let organ_op = Self::find_organ_by_organ_code(code).await?;
        Self::find_organ_tree_start_with_organ(organ_op).await
    }
    async fn find_organ_tree_start_with_organ(
        organ_op: Option<BmbpRbacOrgan>,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        if organ_op.is_none() {
            return Err(BmbpError::api("指定的节点不存在".to_string()));
        }
        let organ_info = organ_op.unwrap();

        let mut query_script_sql = OrganScript::query_script();
        let mut query_script_params = BmbpHashMap::new();
        if let Some(organ_code_path) = organ_info.get_organ_code_path() {
            query_script_params.insert(
                "organ_code_path".to_string(),
                BmbpValue::from(format!("{}%", organ_code_path)),
            );
            query_script_sql.filter("organ_code_path = #{organ_code_path}");
        } else {
            return Err(BmbpError::api(
                "指定的节点数据异常,请联系管理员".to_string(),
            ));
        }

        let organ_list_op =
            OrganDao::find_organ_tree(&query_script_sql.to_script(), &query_script_params).await?;
        match organ_list_op {
            Some(organ_list) => {
                let organ_tree = TreeBuilder::build::<BmbpRbacOrgan>(organ_list);
                Ok(Some(organ_tree))
            }
            None => Ok(None),
        }
    }

    /// 分页查询组织列表
    pub async fn find_organ_page(
        page_params: &PageParams<OrganQueryParam>,
    ) -> BmbpResp<PageVo<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = OrganScript::query_script();
        if let Some(params) = page_params.get_params() {
            if let Some(organ_parent_code) = params.get_organ_parent_code() {
                if !organ_parent_code.is_empty() {
                    script_param.insert(
                        "organ_parent_code".to_string(),
                        BmbpValue::from(organ_parent_code),
                    );
                    query_script.filter("organ_parent_code = #{organ_parent_code}");
                }
            }

            if let Some(organ_title) = params.get_organ_title() {
                if !organ_title.is_empty() {
                    script_param.insert(
                        "organ_title".to_string(),
                        BmbpValue::from(format!("%{}%", organ_title)),
                    );
                    query_script.filter("organ_title like #{organ_title}");
                }
            }
        }

        OrganDao::find_organ_page(
            &query_script.to_script(),
            &script_param,
            page_params.get_page_no(),
            page_params.get_page_size(),
        )
        .await
    }

    pub async fn find_organ_page_by_parent(
        parent: &String,
        page_params: &mut PageParams<OrganQueryParam>,
    ) -> BmbpResp<PageVo<BmbpRbacOrgan>> {
        if let Some(params) = page_params.get_mut_params() {
            params.set_organ_parent_code(parent.to_string());
        } else {
            let mut organ_params = OrganQueryParam::new();
            organ_params.set_organ_parent_code(parent.to_string());
            page_params.set_params(organ_params);
        }
        Self::find_organ_page(page_params).await
    }

    /// 查询组织列表
    pub async fn find_organ_list(params: &OrganQueryParam) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = OrganScript::query_script();
        if let Some(organ_title) = params.get_organ_title() {
            if !organ_title.is_empty() {
                script_param.insert(
                    "organ_title".to_string(),
                    BmbpValue::from(format!("%{}%", organ_title)),
                );
                query_script.filter("organ_title like #{organ_title}");
            }
        }
        if let Some(organ_parent_code) = params.get_organ_parent_code() {
            if !organ_parent_code.is_empty() {
                script_param.insert(
                    "organ_parent_code".to_string(),
                    BmbpValue::from(organ_parent_code),
                );
                query_script.filter("organ_parent_code = #{organ_parent_code}");
            }
        }
        OrganDao::find_organ_list(&query_script.to_script(), &script_param).await
    }
    pub async fn find_organ_list_by_parent(
        parent: &String,
        params: &OrganQueryParam,
    ) -> BmbpResp<Vec<BmbpRbacOrgan>> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
    /// 查询组织详情-> 多个参数
    pub async fn find_organ_one(organ: &BmbpRbacOrgan) -> BmbpResp<Option<BmbpRbacOrgan>> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
    /// 查询组织详情-通过R_ID
    pub async fn find_organ_by_id(r_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = OrganScript::query_script();
        script_param.insert("record_id".to_string(), BmbpValue::from(r_id));
        query_script.filter("record_id = #{record_id}");
        OrganDao::find_organ_info(&query_script.to_script(), &script_param).await
    }
    /// 查询组织详情-通过ORGAN-CODE
    pub async fn find_organ_by_organ_code(organ_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
    /// 保存组织
    pub async fn save_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
    /// 新增组织
    pub async fn insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }
    /// 更新组织
    pub async fn update_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Err(BmbpError::api("接口未实现".to_string()))
    }

    /// 更新组织状态
    pub async fn update_organ_status(id: String, status: String) -> BmbpResp<usize> {
        Ok(0)
    }
    /// 更新组织上级
    pub async fn update_organ_parent(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Ok(0)
    }
    /// 删除组织
    pub async fn remove_organ_by_id(id: String) -> BmbpResp<usize> {
        Ok(0)
    }
    /// 删除组织
    pub async fn remove_organ(params: &OrganQueryParam) -> BmbpResp<usize> {
        Ok(0)
    }
}

/// 校验逻辑
#[allow(dead_code)]
#[allow(unused)]
impl OrganService {
    /// 保存时的数据校验
    pub fn valid_insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否包含相同的数据关联
    pub async fn check_same_data_id(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否包含相同组织
    pub async fn check_same_organ_title(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否包含下级
    pub async fn check_organ_has_children(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否关联用户
    pub async fn check_organ_has_user(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否关联业务
    #[allow(dead_code)]
    pub async fn check_organ_has_data(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
}
