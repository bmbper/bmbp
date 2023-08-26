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

    pub(crate) async fn find_organ_tree_with_out_id(
        with_out_organ_id: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let organ_code = {
            if let Some(organ) = Self::find_organ_by_id(with_out_organ_id).await? {
                organ.get("organCode").unwrap().to_string()
            } else {
                "".to_string()
            }
        };
        tracing::info!("排除节点：{}", organ_code);
        let mut script = OrganScript::query_script();
        script.filter("organ_code_path not like concat('%/',#{organCode}::TEXT,'/%')");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("organCode".to_string(), BmbpValue::from(organ_code));
        let organ_tree_rs = OrganDao::find_organ_list(&script.to_script(), &script_params).await?;
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
        if organ_op.is_none() {
            return Err(BmbpError::service("请指定查询信息"));
        }
        let organ = organ_op.unwrap();
        if !is_empty_prop(&organ, "recordId") {
            return Self::find_organ_by_id(&organ.get("recordId").unwrap().to_string()).await;
        }
        if !is_empty_prop(&organ, "organCode") {
            return Self::find_organ_by_organ_code(&organ.get("organCode").unwrap().to_string())
                .await;
        }
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询组织详情-通过R_ID
    pub async fn find_organ_by_id(r_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = OrganScript::query_script();
        script.filter("record_id = #{recordId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(r_id));
        OrganDao::find_organ_info(&script.to_script(), &script_params).await
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
        let mut script = OrganScript::query_script();
        script.filter("organ_data_id = #{organDataId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("organDataId".to_string(), BmbpValue::from(organ_data_id));
        OrganDao::find_organ_info(&script.to_script(), &script_params).await
    }

    /// save_organ 保存组织
    pub(crate) async fn save_organ(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_organ(params).await
        } else {
            Self::update_organ(params).await
        }
    }

    /// 插入组织
    pub(crate) async fn insert_organ(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        /// 组织数据校验
        if let Some(err) = Self::valid_insert_organ_data(params) {
            return Err(err);
        }

        /// 新增默认值
        add_insert_default_value(params);

        /// 计算组织树信息
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

        /// 判断是否有重复组织
        if !Self::check_same_organ_title(params, false).await? {
            return Err(BmbpError::service("已存在相同名称的组织"));
        }
        let _ = Self::check_same_organ_organ_code(params).await?;
        let _ = Self::check_same_organ_record_id(params).await?;
        let _ = Self::check_same_organ_data_id(params).await?;

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
                // 继承上级节点状态
                let record_status = parent_organ.get("recordStatus").unwrap();
                params.insert("recordStatus".to_string(), record_status.clone());
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

        // 旧标题
        let mut old_organ_title = "".to_string();
        let mut old_organ_title_path = "".to_string();
        let mut old_organ_parent = "".to_string();
        // 新标题
        let new_organ_title = params.get("organTitle").unwrap().to_string();
        // 旧标题路径
        if let Some(organ) =
            Self::find_organ_by_id(&params.get("recordId").unwrap().to_string()).await?
        {
            old_organ_title = organ.get("organTitle").unwrap().to_string();
            old_organ_title_path = organ.get("organTitlePath").unwrap().to_string();
            old_organ_parent = organ.get("organParentCode").unwrap().to_string();
        } else {
            return Err(BmbpError::service("指定更新的组织不存在！"));
        }

        let mut script = OrganScript::update_script();
        add_update_default_value(params);
        if !is_empty_prop(params, "recordNum") {
            script.set_value("record_num", "#{recordNum}");
        }
        script.filter("record_id = #{recordId}");
        let mut row_count = 0;
        // 组织名称发生变更，更新底层路径
        if !is_empty_prop(params, "organTitle") && !new_organ_title.eq(&old_organ_title) {
            script.set_value("organ_title", "#{organTitle}");

            // 计算OrganTitlePath
            let old_title_length = old_organ_title.len();
            let old_title_path_length = old_organ_title_path.len();
            let sub_length = old_title_path_length - old_title_length - 1;
            // 计算新路径
            let parent_path = &old_organ_title_path[0..sub_length];
            let new_organ_title_path = format!("{}{}/", parent_path, new_organ_title);
            script.set_value("organ_title_path", "#{organTitlePath}");
            params.insert(
                "organTitlePath".to_string(),
                BmbpValue::from(new_organ_title_path),
            );
            row_count = OrganDao::update(&script.to_script(), &params).await?;
            let record_id = params.get("recordId").unwrap().to_string();
            row_count = row_count
                + Self::update_organ_parent_by_record_id(&record_id, &old_organ_parent).await?;
        } else {
            row_count = OrganDao::update(&script.to_script(), &params).await?;
        }

        Ok(row_count)
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
        if let Some(organ) = Self::find_organ_by_id(id).await? {
            if is_empty_prop(&organ, "organCodePath") {
                return Err(BmbpError::service(
                    format!("异常的的组织数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            if is_empty_prop(&organ, "organCode") {
                return Err(BmbpError::service(
                    format!("异常的的组织数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            tracing::info!("organ:{:#?}", organ.get("organCode").unwrap().clone());
            let mut script = OrganScript::update_status_script();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordStatus".to_string(), BmbpValue::from(status));
            match status.as_str() {
                "-1" => {
                    //停用 向下停用
                    script.filter("organ_code_path like concat(#{organCodePath}::TEXT,'%')");
                    script_params.insert(
                        "organCodePath".to_string(),
                        organ.get("organCodePath").unwrap().clone(),
                    );
                }
                _ => {
                    let organ_code_path = organ.get("organCodePath").unwrap().to_string();
                    let organ_code_array: Vec<String> =
                        organ_code_path.split("/").map(|x| x.to_string()).collect();
                    let trim_organ_code = organ_code_array
                        .iter()
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    script.filter("organ_code = ANY(#{organCode})");
                    script_params.insert("organCode".to_string(), BmbpValue::from(trim_organ_code));
                }
            }

            OrganDao::update(&script.to_script(), &script_params).await
        } else {
            return Err(BmbpError::service(
                format!("请定的组织无效:{}", id).as_str(),
            ));
        }
    }
    /// 更新组织上级
    pub async fn update_organ_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        let current_organ = {
            if let Some(organ) = Self::find_organ_by_id(record_id).await? {
                organ
            } else {
                return Err(BmbpError::service("待变更的组织节点不存在"));
            }
        };

        let new_parent_organ = {
            if let Some(organ) = Self::find_organ_by_organ_code(parent).await? {
                organ
            } else {
                return Err(BmbpError::service("指定的上级组织节点不存在"));
            }
        };
        // 当前节点属性
        let current_organ_code = current_organ.get("organCode").unwrap().to_string();
        let current_organ_title = current_organ.get("organTitle").unwrap().to_string();
        let current_organ_code_path = current_organ.get("organCodePath").unwrap().to_string();
        let current_organ_title_path = current_organ.get("organTitlePath").unwrap().to_string();
        // 计算原父点属性
        let organ_title_length = format!("{}/", current_organ_title).len();
        let organ_code_length = format!("{}/", current_organ_code).len();
        let current_organ_code_path_length = current_organ_code_path.len();
        let current_organ_title_path_length = current_organ_title_path.len();
        let old_parent_organ_cdoe_path =
            &current_organ_code_path[0..(current_organ_code_path_length - organ_code_length)];
        let old_parent_organ_title_path =
            &current_organ_title_path[0..(current_organ_title_path_length - organ_title_length)];
        let parent_organ_code_path = new_parent_organ.get("organCodePath").unwrap().to_string();
        let parent_organ_title_path = new_parent_organ.get("organTitlePath").unwrap().to_string();
        /// 还没有事务
        // 更新organPathentCode
        let update_parent_script = OrganScript::update_parent_script();
        let mut update_parent_script_params = BmbpHashMap::new();
        update_parent_script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        update_parent_script_params.insert("organParentCode".to_string(), BmbpValue::from(parent));

        // 更新自身及下级标题路径
        let mut update_organ_title_path_script = OrganScript::update_title_path_script();
        let mut update_organ_title_path_script_params = BmbpHashMap::new();
        update_organ_title_path_script_params.insert(
            "newOrganTitlePath".to_string(),
            BmbpValue::from(parent_organ_title_path),
        );
        update_organ_title_path_script_params.insert(
            "oldOrganParentTitlePath".to_string(),
            BmbpValue::from(old_parent_organ_title_path),
        );
        update_organ_title_path_script_params.insert(
            "currentOrganTitlePath".to_string(),
            BmbpValue::from(current_organ_title_path),
        );

        // 更新自身及下级编码路径
        let mut update_organ_code_path_script = OrganScript::update_code_path_script();

        let mut update_organ_code_path_script_params = BmbpHashMap::new();
        update_organ_code_path_script_params.insert(
            "newOrganCodePath".to_string(),
            BmbpValue::from(parent_organ_code_path),
        );
        update_organ_code_path_script_params.insert(
            "oldOrganParentCodePath".to_string(),
            BmbpValue::from(old_parent_organ_cdoe_path),
        );
        update_organ_code_path_script_params.insert(
            "currentOrganCodePath".to_string(),
            BmbpValue::from(current_organ_code_path),
        );
        let mut row_count = OrganDao::update(
            &update_parent_script.to_script(),
            &update_parent_script_params,
        )
        .await?;
        row_count = row_count
            + OrganDao::update(
                &update_organ_title_path_script.to_script(),
                &update_organ_title_path_script_params,
            )
            .await?;
        row_count = row_count
            + OrganDao::update(
                &update_organ_code_path_script.to_script(),
                &update_organ_code_path_script_params,
            )
            .await?;
        Ok(row_count)
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
impl OrganService {
    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_organ_code(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }
    // 判断是否包含相同的数据关联
    pub async fn check_same_organ_record_id(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }

    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_data_id(organ: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否包含相同组织
    pub async fn check_same_organ_title(
        organ: &mut BmbpHashMap,
        is_update: bool,
    ) -> BmbpResp<bool> {
        if is_empty_prop(organ, "organTitle") {
            return Err(BmbpError::service("组织名称不允许为空"));
        }
        if is_empty_prop(organ, "organParentCode") {
            return Err(BmbpError::service("组织上级不允许为空"));
        }

        let mut script = OrganScript::query_script();
        script.filter("organ_title = #{organTitle}");
        script.filter("organ_parent_code = #{organParentCode}");
        if is_update {
            if is_empty_prop(organ, "recordId") {
                return Err(BmbpError::service("请指定待更新的组织标识"));
            }
            script.filter("record_id != #{recordId}");
        }
        let organ_rs = OrganDao::find_organ_info(&script.to_script(), &organ).await?;
        Ok(organ_rs.is_none())
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
