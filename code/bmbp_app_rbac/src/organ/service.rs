use super::{dao::OrganDao, script::OrganScript};
use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, FieldValidRule, PageParams, PageVo, ValidRule,
    ValidType, ROOT_TREE_NODE,
};
use bmbp_app_utils::{
    add_insert_default_value, add_update_default_value, is_empty_prop, simple_uuid_upper,
    valid_field_rule, valid_field_rule_slice, HashMapTreeBuilder,
};

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
            query_script.order_by("organ_parent_code asc");
            query_script.order_by("record_num asc");
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
        params: &PageParams<BmbpHashMap>,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let mut query_script = OrganScript::query_script();
        query_script.order_by("organ_parent_code asc");
        query_script.order_by("record_num asc");
        let mut query_params = BmbpHashMap::new();
        if let Some(organ_params) = params.get_params() {
            if !is_empty_prop(organ_params, "organParentCode") {
                let parent_code = organ_params.get("organParentCode").unwrap().to_string();
                if let Some(parent_node) = Self::find_organ_by_organ_code(&parent_code).await? {
                    let organ_code_path = parent_node.get("organCodePath").unwrap();
                    query_script.filter("organ_code_path like concat(#{organCodePath}::TEXT,'%')");
                    query_params.insert("organCodePath".to_string(), organ_code_path.clone());
                } else {
                    return Err(BmbpError::service("上级组织不存在"));
                }
            }
        }
        let page_vo = OrganDao::find_organ_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
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
        let mut script = OrganScript::query_script();
        script.filter("organ_code = #{organCode}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("organCode".to_string(), BmbpValue::from(organ_code));
        OrganDao::find_organ_info(&script.to_script(), &script_params).await
    }
    /// 查询组织详情-通过ORGAN_DATA_ID
    pub async fn find_organ_by_organ_data_id(
        organ_data_id: &String,
    ) -> BmbpResp<Option<BmbpHashMap>> {
        Err(BmbpError::service("服务未实现"))
    }

    pub(crate) async fn save_organ(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_organ(params).await
        } else {
            Self::update_organ(params).await
        }
    }
    pub(crate) async fn insert_organ(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if let Some(err) = Self::valid_insert_organ_data(params) {
            return Err(err);
        }
        add_insert_default_value(params);
        let mut organ_parent_code = ROOT_TREE_NODE.to_string();
        if is_empty_prop(params, "organParentCode") {
            params.insert(
                "organParentCode".to_string(),
                BmbpValue::from(ROOT_TREE_NODE),
            );
        } else {
            organ_parent_code = params.get("organParentCode").unwrap().to_string();
        }

        if is_empty_prop(params, "organCode") {
            params.insert(
                "organCode".to_string(),
                BmbpValue::from(simple_uuid_upper()),
            );
        }
        if is_empty_prop(params, "organDataId") {
            params.insert(
                "organDataId".to_string(),
                BmbpValue::from(simple_uuid_upper()),
            );
        }

        // 当前编码
        let current_code = params.get("organCode").unwrap().to_string();
        // 当前名称
        let current_title = params.get("organTitle").unwrap().to_string();

        let mut current_organ_code_path = "".to_string();
        let mut current_organ_title_path = "".to_string();

        // 根节点为父节点
        if organ_parent_code.eq(&ROOT_TREE_NODE.to_string()) {
            current_organ_code_path = format!("/{}/", current_code);
            current_organ_title_path = format!("/{}/", current_title);
        } else {
            if let Some(parent_organ) = Self::find_organ_by_organ_code(&organ_parent_code).await? {
                let parent_code_path = parent_organ.get("organCodePath").unwrap().to_string();
                let parent_title_path = parent_organ.get("organTitlePath").unwrap().to_string();
                current_organ_code_path = format!("{}{}/", parent_code_path, current_code);
                current_organ_title_path = format!("{}{}/", parent_title_path, current_title);
            } else {
                return Err(BmbpError::service("指定的上级组织不存在"));
            }
        }

        params.insert(
            "organCodePath".to_string(),
            BmbpValue::from(current_organ_code_path),
        );
        params.insert(
            "organTitlePath".to_string(),
            BmbpValue::from(current_organ_title_path),
        );

        let script = OrganScript::insert_script();
        OrganDao::insert(&script.to_script(), params).await
    }
    pub(crate) async fn update_organ(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        let record_id_valid_rule = FieldValidRule(
            "recordId".to_string(),
            ValidRule(ValidType::NotEmpty, "主键不允许为空".to_string()),
        );
        if let Some(err) = valid_field_rule(params, &record_id_valid_rule) {
            return Err(err);
        }
        let mut script = OrganScript::update_script();
        add_update_default_value(params);
        if !is_empty_prop(params, "organTitle") {
            script.set_value("organ_title", "#{organTitle}");
        }
        if !is_empty_prop(params, "recordNum") {
            script.set_value("record_num", "#{record_num}");
        }

        OrganDao::update(&script.to_script(), params).await
    }

    /// 验证应用新增数据
    fn valid_insert_organ_data(
        params: &mut std::collections::HashMap<String, BmbpValue>,
    ) -> Option<bmbp_app_common::BmbpError> {
        let valid_rule = vec![
            FieldValidRule(
                "organTitle".to_string(),
                ValidRule(ValidType::NotEmpty, "组织名称不能为空!".to_string()),
            ),
            FieldValidRule(
                "organType".to_string(),
                ValidRule(ValidType::NotEmpty, r#"组织类型不能为空!"#.to_string()),
            ),
        ];
        valid_field_rule_slice(params, valid_rule.as_slice())
    }

    /// 更新组织状态
    pub async fn update_organ_status(id: &String, status: &String) -> BmbpResp<usize> {
        let script = OrganScript::update_status_script();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(id));
        script_params.insert("recordStatus".to_string(), BmbpValue::from(status));
        OrganDao::delete(&script.to_script(), &script_params).await
    }
    /// 更新组织上级
    pub async fn update_organ_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
    /// 删除组织
    pub async fn remove_organ_by_id(id: &String) -> BmbpResp<usize> {
        let script = OrganScript::delete_script_by_id();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(id));
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
