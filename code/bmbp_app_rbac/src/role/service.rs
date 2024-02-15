use super::{dao::RoleDao, script::RoleScript};
use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, FieldValidRule, BmbpPageParam, PageVo, ValidRule,
    ValidType, ROOT_TREE_NODE,
};
use bmbp_app_utils::{
    add_insert_default_value, add_update_default_value, is_empty_prop, simple_uuid_upper,
    valid_field_rule, valid_field_rule_slice, HashMapTreeBuilder,
};

/// 服务声明
pub struct RoleService();

/// CURD 逻辑
#[allow(dead_code)]
#[allow(unused)]
impl RoleService {
    /// 查询角色树
    pub async fn find_role_tree(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let role_code_path = { "".to_string() };
        Self::find_role_tree_by_role_code_path(&role_code_path).await
    }
    /// 查询指定ID下的角色机构树
    pub async fn find_role_tree_start_with_id(id: &String) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_role_by_id(id).await?;
        Self::find_role_tree_start_with_role(current_node).await
    }
    ///查询指定编码的角色机构树
    pub async fn find_role_tree_start_with_code(
        code: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_role_by_role_code(code).await?;
        Self::find_role_tree_start_with_role(current_node).await
    }
    /// 查询指定角色的角色机构树
    async fn find_role_tree_start_with_role(
        role_op: Option<BmbpHashMap>,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        if role_op.is_none() {
            return Err(BmbpError::service("未找到指定的角色信息"));
        }
        let current_role = role_op.unwrap();
        if let Some(role_code_path) = current_role.get("roleCodePath") {
            Self::find_role_tree_by_role_code_path(&role_code_path.to_string()).await
        } else {
            Err(BmbpError::service("指定的结点数据异常，请联系管理员"))
        }
    }
    /// 查询指定编码路径的角色机构树
    pub async fn find_role_tree_by_role_code_path(
        role_code_path: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let mut query_script = RoleScript::query_script();
        let mut query_params = BmbpHashMap::new();
        if !role_code_path.is_empty() {
            query_script.filter("role_code_path like concat(#{roleCodePath},'%'");
            query_params.insert("roleCodePath".to_string(), BmbpValue::from(role_code_path));
            query_script.order_by("record_num asc");
        }
        let role_tree_rs =
            RoleDao::find_role_list(&query_script.to_script(), &query_params).await?;
        match role_tree_rs {
            Some(role_list) => Ok(Some(HashMapTreeBuilder::build_tree_by_name(
                role_list, "role",
            ))),
            None => Ok(None),
        }
    }

    pub(crate) async fn find_role_tree_with_out_id(
        with_out_role_id: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let role_code = {
            if let Some(role) = Self::find_role_by_id(with_out_role_id).await? {
                role.get("roleCode").unwrap().to_string()
            } else {
                "".to_string()
            }
        };
        tracing::info!("排除节点：{}", role_code);
        let mut script = RoleScript::query_script();
        script.filter("role_code_path not like concat('%/',#{roleCode}::TEXT,'/%')");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("roleCode".to_string(), BmbpValue::from(role_code));
        let role_tree_rs = RoleDao::find_role_list(&script.to_script(), &script_params).await?;
        match role_tree_rs {
            Some(role_list) => Ok(Some(HashMapTreeBuilder::build_tree_by_name(
                role_list, "role",
            ))),
            None => Ok(None),
        }
    }

    /// 分页查询角色列表
    pub async fn find_role_page(params: &BmbpPageParam<BmbpHashMap>) -> BmbpResp<PageVo<BmbpHashMap>> {
        let mut query_script = RoleScript::query_script();
        query_script.order_by("role_parent_code asc");
        query_script.order_by("record_num asc");
        let mut query_params = BmbpHashMap::new();
        if let Some(role_params) = params.get_params() {
            if !is_empty_prop(role_params, "roleParentCode") {
                let parent_code = role_params.get("roleParentCode").unwrap().to_string();
                if let Some(parent_node) = Self::find_role_by_role_code(&parent_code).await? {
                    let role_code_path = parent_node.get("roleCodePath").unwrap();
                    query_script.filter("role_code_path like concat(#{roleCodePath}::TEXT,'%')");
                    query_params.insert("roleCodePath".to_string(), role_code_path.clone());
                } else {
                    return Err(BmbpError::service("上级角色不存在"));
                }
            }
        }
        let page_vo = RoleDao::find_role_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }

    /// 查询角色列表
    pub async fn find_role_list(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }
    pub async fn find_role_list_by_parent(
        parent: &String,
        params: &mut BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 查询角色详情-> 多个参数
    pub async fn find_role_one(role_op: Option<BmbpHashMap>) -> BmbpResp<Option<BmbpHashMap>> {
        if role_op.is_none() {
            return Err(BmbpError::service("请指定查询信息"));
        }
        let role = role_op.unwrap();
        if !is_empty_prop(&role, "recordId") {
            return Self::find_role_by_id(&role.get("recordId").unwrap().to_string()).await;
        }
        if !is_empty_prop(&role, "roleCode") {
            return Self::find_role_by_role_code(&role.get("roleCode").unwrap().to_string()).await;
        }
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询角色详情-通过R_ID
    pub async fn find_role_by_id(r_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = RoleScript::query_script();
        script.filter("record_id = #{recordId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(r_id));
        RoleDao::find_role_info(&script.to_script(), &script_params).await
    }
    /// 查询角色详情-通过ORGAN-CODE
    pub async fn find_role_by_role_code(role_code: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = RoleScript::query_script();
        script.filter("role_code = #{roleCode}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("roleCode".to_string(), BmbpValue::from(role_code));
        RoleDao::find_role_info(&script.to_script(), &script_params).await
    }
    /// 查询角色详情-通过ORGAN_DATA_ID
    pub async fn find_role_by_role_data_id(role_data_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = RoleScript::query_script();
        script.filter("role_data_id = #{roleDataId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("roleDataId".to_string(), BmbpValue::from(role_data_id));
        RoleDao::find_role_info(&script.to_script(), &script_params).await
    }

    /// save_role 保存角色
    pub(crate) async fn save_role(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_role(params).await
        } else {
            Self::update_role(params).await
        }
    }

    /// 插入角色
    pub(crate) async fn insert_role(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        /// 角色数据校验
        if let Some(err) = Self::valid_insert_role_data(params) {
            return Err(err);
        }

        /// 新增默认值
        add_insert_default_value(params);

        /// 计算角色树信息
        let mut role_parent_code = ROOT_TREE_NODE.to_string();
        if is_empty_prop(params, "roleParentCode") {
            params.insert(
                "roleParentCode".to_string(),
                BmbpValue::from(ROOT_TREE_NODE),
            );
        } else {
            role_parent_code = params.get("roleParentCode").unwrap().to_string();
        }

        if is_empty_prop(params, "roleCode") {
            params.insert("roleCode".to_string(), BmbpValue::from(simple_uuid_upper()));
        }
        if is_empty_prop(params, "roleDataId") {
            params.insert(
                "roleDataId".to_string(),
                BmbpValue::from(simple_uuid_upper()),
            );
        }

        // 当前编码
        let current_code = params.get("roleCode").unwrap().to_string();
        // 当前名称
        let current_title = params.get("roleTitle").unwrap().to_string();

        let mut current_role_code_path = "".to_string();
        let mut current_role_title_path = "".to_string();

        /// 判断是否有重复角色
        if Self::has_same_title_role(params, false).await? {
            return Err(BmbpError::service("已存在相同名称的角色"));
        }
        if Self::has_same_role_code_role(params).await? {
            return Err(BmbpError::service("已存在相同编码的角色"));
        }
        if Self::has_same_record_id_role(params).await? {
            return Err(BmbpError::service("已存在相同主简的角色"));
        }
        if Self::has_same_data_id_role(params).await? {
            return Err(BmbpError::service("已存在相同数据标识的角色"));
        }

        // 根节点为父节点
        if role_parent_code.eq(&ROOT_TREE_NODE.to_string()) {
            current_role_code_path = format!("/{}/", current_code);
            current_role_title_path = format!("/{}/", current_title);
        } else {
            if let Some(parent_role) = Self::find_role_by_role_code(&role_parent_code).await? {
                let parent_code_path = parent_role.get("roleCodePath").unwrap().to_string();
                let parent_title_path = parent_role.get("roleTitlePath").unwrap().to_string();
                current_role_code_path = format!("{}{}/", parent_code_path, current_code);
                current_role_title_path = format!("{}{}/", parent_title_path, current_title);
                // 继承上级节点状态
                let record_status = parent_role.get("recordStatus").unwrap();
                params.insert("recordStatus".to_string(), record_status.clone());
            } else {
                return Err(BmbpError::service("指定的上级角色不存在"));
            }
        }

        params.insert(
            "roleCodePath".to_string(),
            BmbpValue::from(current_role_code_path),
        );
        params.insert(
            "roleTitlePath".to_string(),
            BmbpValue::from(current_role_title_path),
        );

        let script = RoleScript::insert_script();
        RoleDao::insert(&script.to_script(), params).await
    }
    pub(crate) async fn update_role(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        let record_id_valid_rule = FieldValidRule(
            "recordId".to_string(),
            ValidRule(ValidType::NotEmpty, "主键不允许为空".to_string()),
        );
        if let Some(err) = valid_field_rule(params, &record_id_valid_rule) {
            return Err(err);
        }

        // 旧标题
        let mut old_role_title = "".to_string();
        let mut old_role_title_path = "".to_string();
        let mut old_role_parent = "".to_string();
        // 新标题
        let new_role_title = params.get("roleTitle").unwrap().to_string();
        // 旧标题路径
        if let Some(role) =
            Self::find_role_by_id(&params.get("recordId").unwrap().to_string()).await?
        {
            old_role_title = role.get("roleTitle").unwrap().to_string();
            old_role_title_path = role.get("roleTitlePath").unwrap().to_string();
            old_role_parent = role.get("roleParentCode").unwrap().to_string();
        } else {
            return Err(BmbpError::service("指定更新的角色不存在！"));
        }

        let mut script = RoleScript::update_script();
        add_update_default_value(params);
        if !is_empty_prop(params, "recordNum") {
            script.set_value("record_num", "#{recordNum}");
        }
        script.filter("record_id = #{recordId}");
        let mut row_count = 0;
        // 角色名称发生变更，更新底层路径
        if !is_empty_prop(params, "roleTitle") && !new_role_title.eq(&old_role_title) {
            script.set_value("role_title", "#{roleTitle}");

            // 计算RoleTitlePath
            let old_title_length = old_role_title.len();
            let old_title_path_length = old_role_title_path.len();
            let sub_length = old_title_path_length - old_title_length - 1;
            // 计算新路径
            let parent_path = &old_role_title_path[0..sub_length];
            let new_role_title_path = format!("{}{}/", parent_path, new_role_title);
            script.set_value("role_title_path", "#{roleTitlePath}");
            params.insert(
                "roleTitlePath".to_string(),
                BmbpValue::from(new_role_title_path),
            );
            row_count = RoleDao::update(&script.to_script(), &params).await?;
            let record_id = params.get("recordId").unwrap().to_string();
            row_count = row_count
                + Self::update_role_parent_by_record_id(&record_id, &old_role_parent).await?;
        } else {
            row_count = RoleDao::update(&script.to_script(), &params).await?;
        }

        Ok(row_count)
    }

    /// 验证应用新增数据
    fn valid_insert_role_data(
        params: &mut std::collections::HashMap<String, BmbpValue>,
    ) -> Option<bmbp_app_common::BmbpError> {
        let valid_rule = vec![
            FieldValidRule(
                "roleTitle".to_string(),
                ValidRule(ValidType::NotEmpty, "角色名称不能为空!".to_string()),
            ),
            FieldValidRule(
                "roleType".to_string(),
                ValidRule(ValidType::NotEmpty, r#"角色类型不能为空!"#.to_string()),
            ),
        ];
        valid_field_rule_slice(params, valid_rule.as_slice())
    }

    /// 更新角色状态
    pub async fn update_role_status(id: &String, status: &String) -> BmbpResp<usize> {
        if let Some(role) = Self::find_role_by_id(id).await? {
            if is_empty_prop(&role, "roleCodePath") {
                return Err(BmbpError::service(
                    format!("异常的的角色数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            if is_empty_prop(&role, "roleCode") {
                return Err(BmbpError::service(
                    format!("异常的的角色数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            tracing::info!("role:{:#?}", role.get("roleCode").unwrap().clone());
            let mut script = RoleScript::update_status_script();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordStatus".to_string(), BmbpValue::from(status));
            match status.as_str() {
                "-1" => {
                    //停用 向下停用
                    script.filter("role_code_path like concat(#{roleCodePath}::TEXT,'%')");
                    script_params.insert(
                        "roleCodePath".to_string(),
                        role.get("roleCodePath").unwrap().clone(),
                    );
                }
                _ => {
                    let role_code_path = role.get("roleCodePath").unwrap().to_string();
                    let role_code_array: Vec<String> =
                        role_code_path.split("/").map(|x| x.to_string()).collect();
                    let trim_role_code = role_code_array
                        .iter()
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    script.filter("role_code = ANY(#{roleCode})");
                    script_params.insert("roleCode".to_string(), BmbpValue::from(trim_role_code));
                }
            }

            RoleDao::update(&script.to_script(), &script_params).await
        } else {
            return Err(BmbpError::service(
                format!("请定的角色无效:{}", id).as_str(),
            ));
        }
    }
    /// 更新角色上级
    pub async fn update_role_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        let current_role = {
            if let Some(role) = Self::find_role_by_id(record_id).await? {
                role
            } else {
                return Err(BmbpError::service("待变更的角色节点不存在"));
            }
        };

        let new_parent_role = {
            if let Some(role) = Self::find_role_by_role_code(parent).await? {
                role
            } else {
                return Err(BmbpError::service("指定的上级角色节点不存在"));
            }
        };
        // 当前节点属性
        let current_role_code = current_role.get("roleCode").unwrap().to_string();
        let current_role_title = current_role.get("roleTitle").unwrap().to_string();
        let current_role_code_path = current_role.get("roleCodePath").unwrap().to_string();
        let current_role_title_path = current_role.get("roleTitlePath").unwrap().to_string();
        // 计算原父点属性
        let role_title_length = format!("{}/", current_role_title).len();
        let role_code_length = format!("{}/", current_role_code).len();
        let current_role_code_path_length = current_role_code_path.len();
        let current_role_title_path_length = current_role_title_path.len();
        let old_parent_role_cdoe_path =
            &current_role_code_path[0..(current_role_code_path_length - role_code_length)];
        let old_parent_role_title_path =
            &current_role_title_path[0..(current_role_title_path_length - role_title_length)];
        let parent_role_code_path = new_parent_role.get("roleCodePath").unwrap().to_string();
        let parent_role_title_path = new_parent_role.get("roleTitlePath").unwrap().to_string();
        /// 还没有事务
        // 更新rolePathentCode
        let update_parent_script = RoleScript::update_parent_script();
        let mut update_parent_script_params = BmbpHashMap::new();
        update_parent_script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        update_parent_script_params.insert("roleParentCode".to_string(), BmbpValue::from(parent));

        // 更新自身及下级标题路径
        let mut update_role_title_path_script = RoleScript::update_title_path_script();
        let mut update_role_title_path_script_params = BmbpHashMap::new();
        update_role_title_path_script_params.insert(
            "newRoleTitlePath".to_string(),
            BmbpValue::from(parent_role_title_path),
        );
        update_role_title_path_script_params.insert(
            "oldRoleParentTitlePath".to_string(),
            BmbpValue::from(old_parent_role_title_path),
        );
        update_role_title_path_script_params.insert(
            "currentRoleTitlePath".to_string(),
            BmbpValue::from(current_role_title_path),
        );

        // 更新自身及下级编码路径
        let mut update_role_code_path_script = RoleScript::update_code_path_script();

        let mut update_role_code_path_script_params = BmbpHashMap::new();
        update_role_code_path_script_params.insert(
            "newRoleCodePath".to_string(),
            BmbpValue::from(parent_role_code_path),
        );
        update_role_code_path_script_params.insert(
            "oldRoleParentCodePath".to_string(),
            BmbpValue::from(old_parent_role_cdoe_path),
        );
        update_role_code_path_script_params.insert(
            "currentRoleCodePath".to_string(),
            BmbpValue::from(current_role_code_path),
        );
        let mut row_count = RoleDao::update(
            &update_parent_script.to_script(),
            &update_parent_script_params,
        )
        .await?;
        row_count = row_count
            + RoleDao::update(
                &update_role_title_path_script.to_script(),
                &update_role_title_path_script_params,
            )
            .await?;
        row_count = row_count
            + RoleDao::update(
                &update_role_code_path_script.to_script(),
                &update_role_code_path_script_params,
            )
            .await?;
        Ok(row_count)
    }
    /// 删除角色
    pub async fn remove_role_by_id(id: &String) -> BmbpResp<usize> {
        if let Some(role) = Self::find_role_by_id(id).await? {
            if Self::current_role_has_children(&role).await? {
                return Err(BmbpError::service("指定的角色存在下级节点，无法删除"));
            }

            if Self::currrent_role_has_user(&role).await? {
                return Err(BmbpError::service("指定的角色存在用户，无法删除"));
            }

            if Self::current_role_has_data(&role).await? {
                return Err(BmbpError::service("指定的角色存在业务数据，无法删除"));
            }
            let script = RoleScript::delete_script_by_id();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordId".to_string(), BmbpValue::from(id));
            RoleDao::delete(&script.to_script(), &script_params).await
        } else {
            Err(BmbpError::service("指定的角色不存在，无法删除!"))
        }
    }
    /// 删除角色
    pub async fn remove_role(params: &BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
}

/// 校验逻辑
impl RoleService {
    /// 判断是否包含相同的数据关联
    pub async fn has_same_role_code_role(_role: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    // 判断是否包含相同的数据关联
    pub async fn has_same_record_id_role(_role: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }

    /// 判断是否包含相同的数据关联
    pub async fn has_same_data_id_role(_role: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否包含相同角色
    pub async fn has_same_title_role(role: &mut BmbpHashMap, is_update: bool) -> BmbpResp<bool> {
        if is_empty_prop(role, "roleTitle") {
            return Err(BmbpError::service("角色名称不允许为空"));
        }
        if is_empty_prop(role, "roleParentCode") {
            return Err(BmbpError::service("角色上级不允许为空"));
        }

        let mut script = RoleScript::query_script();
        script.filter("role_title = #{roleTitle}");
        script.filter("role_parent_code = #{roleParentCode}");
        if is_update {
            if is_empty_prop(role, "recordId") {
                return Err(BmbpError::service("请指定待更新的角色标识"));
            }
            script.filter("record_id != #{recordId}");
        }
        let role_rs = RoleDao::find_role_info(&script.to_script(), &role).await?;
        Ok(role_rs.is_some())
    }
    /// 判断是否包含下级
    pub async fn current_role_has_children(_role: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否关联用户
    pub async fn currrent_role_has_user(_role: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否关联业务
    pub async fn current_role_has_data(_role: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
}
