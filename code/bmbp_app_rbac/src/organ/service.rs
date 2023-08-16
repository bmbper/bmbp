use super::{dao::OrganDao, script::OrganScript};
use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, PageParams, PageVo, ROOT_TREE_NODE,
};
use bmbp_app_utils::{is_empty_prop, HashMapTreeBuilder};

/// 服务声明
pub struct OrganService();

/// CURD 逻辑
#[allow(dead_code)]
#[allow(unused)]
impl OrganService {
    /// 查询组织树
    pub async fn find_organ_tree(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let organ_code_path = { "".to_string() };
        Self::find_organ_tree_by_organ_code_path(&organ_code_path).await
    }
    /// 查询指定ID下的组织机构树
    pub async fn find_organ_tree_start_with_id(id: &String) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_organ_by_id(id).await?;
        Self::find_organ_tree_start_with_organ(current_node).await
    }
    ///查询指定编码的组织机构树
    pub async fn find_organ_tree_start_with_code(
        code: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_organ_by_organ_code(code).await?;
        Self::find_organ_tree_start_with_organ(current_node).await
    }
    /// 查询指定组织的组织机构树
    async fn find_organ_tree_start_with_organ(
        organ_op: Option<BmbpHashMap>,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        if organ_op.is_none() {
            return Err(BmbpError::service("未找到指定的组织信息"));
        }
        let current_organ = organ_op.unwrap();
        if let Some(organ_code_path) = current_organ.get("organCodePath") {
            Self::find_organ_tree_by_organ_code_path(&organ_code_path.to_string()).await
        } else {
            Err(BmbpError::service("指定的结点数据异常，请联系管理员"))
        }
    }
    /// 查询指定编码路径的组织机构树
    pub async fn find_organ_tree_by_organ_code_path(
        organ_code_path: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let mut query_script = OrganScript::query_script();
        let mut query_params = BmbpHashMap::new();
        if !organ_code_path.is_empty() {
            query_script.filter("organ_code_path like concat(#{organCodePath},'%'");
            query_params.insert(
                "organCodePath".to_string(),
                BmbpValue::from(organ_code_path),
            );
        }
        let organ_tree_rs =
            OrganDao::find_organ_list(&query_script.to_script(), &query_params).await?;
        match organ_tree_rs {
            Some(organ_list) => Ok(Some(HashMapTreeBuilder::build_tree_by_name(
                organ_list, "organ",
            ))),
            None => Ok(None),
        }
    }

    /// 分页查询组织列表
    pub async fn find_organ_page(
        page_params: &PageParams<BmbpHashMap>,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 查询组织列表
    pub async fn find_organ_list(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }
    pub async fn find_organ_list_by_parent(
        parent: &String,
        params: &mut BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 查询组织详情-> 多个参数
    pub async fn find_organ_one(organ_op: Option<BmbpHashMap>) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询组织详情-通过R_ID
    pub async fn find_organ_by_id(r_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询组织详情-通过ORGAN-CODE
    pub async fn find_organ_by_organ_code(organ_code: &String) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询组织详情-通过ORGAN_DATA_ID
    pub async fn find_organ_by_organ_data_id(
        organ_data_id: &String,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 保存组织
    pub async fn save_organ(organ: &mut BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 新增组织
    pub async fn insert_organ(organ: &mut BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 更新组织状态
    pub async fn update_organ_status(id: String, status: String) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 更新组织上级
    pub async fn update_organ_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 删除组织
    pub async fn remove_organ_by_id(id: String) -> BmbpResp<usize> {
        let script = OrganScript::delete_script_by_id();
        let mut script_params = BmbpHashMap::new();
        let id_vec: Vec<String> = id.split(",").map(|v| format!("'{}'", v)).collect();
        let ids = format!("({})", id_vec.join(","));
        script_params.insert("record_id".to_string(), BmbpValue::from(ids));
        OrganDao::delete(&script.to_script(), &script_params).await
    }
    /// 删除组织
    pub async fn remove_organ(params: &BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
}

/// 校验逻辑
#[allow(dead_code)]
#[allow(unused)]
impl OrganService {
    /// 保存时的数据校验
    pub fn valid_insert_organ(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_organ_code(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Err(BmbpError::service("服务未实现"))
    }
    // 判断是否包含相同的数据关联
    pub async fn check_same_organ_record_id(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_data_id(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 判断是否包含相同组织
    pub async fn check_same_organ_title(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 判断是否包含下级
    pub async fn check_organ_has_children(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否关联用户
    pub async fn check_organ_has_user(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否关联业务
    #[allow(dead_code)]
    pub async fn check_organ_has_data(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }
}
