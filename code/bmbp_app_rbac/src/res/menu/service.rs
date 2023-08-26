use super::{dao::MenuDao, script::MenuScript};
use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, FieldValidRule, PageParams, PageVo, ValidRule,
    ValidType, ROOT_TREE_NODE,
};
use bmbp_app_utils::{
    add_insert_default_value, add_update_default_value, is_empty_prop, simple_uuid_upper,
    valid_field_rule, valid_field_rule_slice, HashMapTreeBuilder,
};

/// 服务声明
pub struct MenuService();

/// CURD 逻辑
#[allow(dead_code)]
#[allow(unused)]
impl MenuService {
    /// 查询菜单树
    pub async fn find_menu_tree(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let menu_code_path = { "".to_string() };
        Self::find_menu_tree_by_menu_code_path(&menu_code_path).await
    }
    /// 查询指定ID下的菜单机构树
    pub async fn find_menu_tree_start_with_id(id: &String) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_menu_by_id(id).await?;
        Self::find_menu_tree_start_with_menu(current_node).await
    }
    ///查询指定编码的菜单机构树
    pub async fn find_menu_tree_start_with_code(
        code: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let current_node = Self::find_menu_by_menu_code(code).await?;
        Self::find_menu_tree_start_with_menu(current_node).await
    }
    /// 查询指定菜单的菜单机构树
    async fn find_menu_tree_start_with_menu(
        menu_op: Option<BmbpHashMap>,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        if menu_op.is_none() {
            return Err(BmbpError::service("未找到指定的菜单信息"));
        }
        let current_menu = menu_op.unwrap();
        if let Some(menu_code_path) = current_menu.get("menuCodePath") {
            Self::find_menu_tree_by_menu_code_path(&menu_code_path.to_string()).await
        } else {
            Err(BmbpError::service("指定的结点数据异常，请联系管理员"))
        }
    }
    /// 查询指定编码路径的菜单机构树
    pub async fn find_menu_tree_by_menu_code_path(
        menu_code_path: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let mut query_script = MenuScript::query_script();
        let mut query_params = BmbpHashMap::new();
        if !menu_code_path.is_empty() {
            query_script.filter("menu_code_path like concat(#{menuCodePath},'%'");
            query_params.insert("menuCodePath".to_string(), BmbpValue::from(menu_code_path));
            query_script.order_by("record_num asc");
        }
        let menu_tree_rs =
            MenuDao::find_menu_list(&query_script.to_script(), &query_params).await?;
        match menu_tree_rs {
            Some(menu_list) => Ok(Some(HashMapTreeBuilder::build_tree_by_name(
                menu_list, "menu",
            ))),
            None => Ok(None),
        }
    }

    pub(crate) async fn find_menu_tree_with_out_id(
        with_out_menu_id: &String,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let menu_code = {
            if let Some(menu) = Self::find_menu_by_id(with_out_menu_id).await? {
                menu.get("menuCode").unwrap().to_string()
            } else {
                "".to_string()
            }
        };
        tracing::info!("排除节点：{}", menu_code);
        let mut script = MenuScript::query_script();
        script.filter("menu_code_path not like concat('%/',#{menuCode}::TEXT,'/%')");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("menuCode".to_string(), BmbpValue::from(menu_code));
        let menu_tree_rs = MenuDao::find_menu_list(&script.to_script(), &script_params).await?;
        match menu_tree_rs {
            Some(menu_list) => Ok(Some(HashMapTreeBuilder::build_tree_by_name(
                menu_list, "menu",
            ))),
            None => Ok(None),
        }
    }

    /// 分页查询菜单列表
    pub async fn find_menu_page(params: &PageParams<BmbpHashMap>) -> BmbpResp<PageVo<BmbpHashMap>> {
        let mut query_script = MenuScript::query_script();
        query_script.order_by("menu_parent_code asc");
        query_script.order_by("record_num asc");
        let mut query_params = BmbpHashMap::new();
        if let Some(menu_params) = params.get_params() {
            if !is_empty_prop(menu_params, "menuParentCode") {
                let parent_code = menu_params.get("menuParentCode").unwrap().to_string();
                if let Some(parent_node) = Self::find_menu_by_menu_code(&parent_code).await? {
                    let menu_code_path = parent_node.get("menuCodePath").unwrap();
                    query_script.filter("menu_code_path like concat(#{menuCodePath}::TEXT,'%')");
                    query_params.insert("menuCodePath".to_string(), menu_code_path.clone());
                } else {
                    return Err(BmbpError::service("上级菜单不存在"));
                }
            }
        }
        let page_vo = MenuDao::find_menu_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }

    /// 查询菜单列表
    pub async fn find_menu_list(params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }
    pub async fn find_menu_list_by_parent(
        parent: &String,
        params: &mut BmbpHashMap,
    ) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        Err(BmbpError::service("服务未实现"))
    }

    /// 查询菜单详情-> 多个参数
    pub async fn find_menu_one(menu_op: Option<BmbpHashMap>) -> BmbpResp<Option<BmbpHashMap>> {
        if menu_op.is_none() {
            return Err(BmbpError::service("请指定查询信息"));
        }
        let menu = menu_op.unwrap();
        if !is_empty_prop(&menu, "recordId") {
            return Self::find_menu_by_id(&menu.get("recordId").unwrap().to_string()).await;
        }
        if !is_empty_prop(&menu, "menuCode") {
            return Self::find_menu_by_menu_code(&menu.get("menuCode").unwrap().to_string()).await;
        }
        Err(BmbpError::service("服务未实现"))
    }
    /// 查询菜单详情-通过R_ID
    pub async fn find_menu_by_id(r_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = MenuScript::query_script();
        script.filter("record_id = #{recordId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(r_id));
        MenuDao::find_menu_info(&script.to_script(), &script_params).await
    }
    /// 查询菜单详情-通过ORGAN-CODE
    pub async fn find_menu_by_menu_code(menu_code: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = MenuScript::query_script();
        script.filter("menu_code = #{menuCode}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("menuCode".to_string(), BmbpValue::from(menu_code));
        MenuDao::find_menu_info(&script.to_script(), &script_params).await
    }
    /// 查询菜单详情-通过ORGAN_DATA_ID
    pub async fn find_menu_by_menu_data_id(menu_data_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = MenuScript::query_script();
        script.filter("menu_data_id = #{menuDataId}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("menuDataId".to_string(), BmbpValue::from(menu_data_id));
        MenuDao::find_menu_info(&script.to_script(), &script_params).await
    }

    /// save_menu 保存菜单
    pub(crate) async fn save_menu(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_menu(params).await
        } else {
            Self::update_menu(params).await
        }
    }

    /// 插入菜单
    pub(crate) async fn insert_menu(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        /// 菜单数据校验
        if let Some(err) = Self::valid_insert_menu_data(params) {
            return Err(err);
        }

        /// 新增默认值
        add_insert_default_value(params);

        /// 计算菜单树信息
        let mut menu_parent_code = ROOT_TREE_NODE.to_string();
        if is_empty_prop(params, "menuParentCode") {
            params.insert(
                "menuParentCode".to_string(),
                BmbpValue::from(ROOT_TREE_NODE),
            );
        } else {
            menu_parent_code = params.get("menuParentCode").unwrap().to_string();
        }

        if is_empty_prop(params, "menuCode") {
            params.insert("menuCode".to_string(), BmbpValue::from(simple_uuid_upper()));
        }
        if is_empty_prop(params, "menuDataId") {
            params.insert(
                "menuDataId".to_string(),
                BmbpValue::from(simple_uuid_upper()),
            );
        }

        // 当前编码
        let current_code = params.get("menuCode").unwrap().to_string();
        // 当前名称
        let current_title = params.get("menuTitle").unwrap().to_string();

        let mut current_menu_code_path = "".to_string();
        let mut current_menu_title_path = "".to_string();

        /// 判断是否有重复菜单
        if Self::has_same_title_menu(params, false).await? {
            return Err(BmbpError::service("已存在相同名称的菜单"));
        }
        if Self::has_same_menu_code_menu(params).await? {
            return Err(BmbpError::service("已存在相同编码的菜单"));
        }
        if Self::has_same_record_id_menu(params).await? {
            return Err(BmbpError::service("已存在相同主简的菜单"));
        }
        if Self::has_same_data_id_menu(params).await? {
            return Err(BmbpError::service("已存在相同数据标识的菜单"));
        }

        // 根节点为父节点
        if menu_parent_code.eq(&ROOT_TREE_NODE.to_string()) {
            current_menu_code_path = format!("/{}/", current_code);
            current_menu_title_path = format!("/{}/", current_title);
        } else {
            if let Some(parent_menu) = Self::find_menu_by_menu_code(&menu_parent_code).await? {
                let parent_code_path = parent_menu.get("menuCodePath").unwrap().to_string();
                let parent_title_path = parent_menu.get("menuTitlePath").unwrap().to_string();
                current_menu_code_path = format!("{}{}/", parent_code_path, current_code);
                current_menu_title_path = format!("{}{}/", parent_title_path, current_title);
                // 继承上级节点状态
                let record_status = parent_menu.get("recordStatus").unwrap();
                params.insert("recordStatus".to_string(), record_status.clone());
            } else {
                return Err(BmbpError::service("指定的上级菜单不存在"));
            }
        }

        params.insert(
            "menuCodePath".to_string(),
            BmbpValue::from(current_menu_code_path),
        );
        params.insert(
            "menuTitlePath".to_string(),
            BmbpValue::from(current_menu_title_path),
        );

        let script = MenuScript::insert_script();
        MenuDao::insert(&script.to_script(), params).await
    }
    pub(crate) async fn update_menu(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        let record_id_valid_rule = FieldValidRule(
            "recordId".to_string(),
            ValidRule(ValidType::NotEmpty, "主键不允许为空".to_string()),
        );
        if let Some(err) = valid_field_rule(params, &record_id_valid_rule) {
            return Err(err);
        }

        // 旧标题
        let mut old_menu_title = "".to_string();
        let mut old_menu_title_path = "".to_string();
        let mut old_menu_parent = "".to_string();
        // 新标题
        let new_menu_title = params.get("menuTitle").unwrap().to_string();
        // 旧标题路径
        if let Some(menu) =
            Self::find_menu_by_id(&params.get("recordId").unwrap().to_string()).await?
        {
            old_menu_title = menu.get("menuTitle").unwrap().to_string();
            old_menu_title_path = menu.get("menuTitlePath").unwrap().to_string();
            old_menu_parent = menu.get("menuParentCode").unwrap().to_string();
        } else {
            return Err(BmbpError::service("指定更新的菜单不存在！"));
        }

        let mut script = MenuScript::update_script();
        add_update_default_value(params);
        if !is_empty_prop(params, "recordNum") {
            script.set_value("record_num", "#{recordNum}");
        }
        script.filter("record_id = #{recordId}");
        let mut row_count = 0;
        // 菜单名称发生变更，更新底层路径
        if !is_empty_prop(params, "menuTitle") && !new_menu_title.eq(&old_menu_title) {
            script.set_value("menu_title", "#{menuTitle}");

            // 计算MenuTitlePath
            let old_title_length = old_menu_title.len();
            let old_title_path_length = old_menu_title_path.len();
            let sub_length = old_title_path_length - old_title_length - 1;
            // 计算新路径
            let parent_path = &old_menu_title_path[0..sub_length];
            let new_menu_title_path = format!("{}{}/", parent_path, new_menu_title);
            script.set_value("menu_title_path", "#{menuTitlePath}");
            params.insert(
                "menuTitlePath".to_string(),
                BmbpValue::from(new_menu_title_path),
            );
            row_count = MenuDao::update(&script.to_script(), &params).await?;
            let record_id = params.get("recordId").unwrap().to_string();
            row_count = row_count
                + Self::update_menu_parent_by_record_id(&record_id, &old_menu_parent).await?;
        } else {
            row_count = MenuDao::update(&script.to_script(), &params).await?;
        }

        Ok(row_count)
    }

    /// 验证应用新增数据
    fn valid_insert_menu_data(
        params: &mut std::collections::HashMap<String, BmbpValue>,
    ) -> Option<bmbp_app_common::BmbpError> {
        let valid_rule = vec![
            FieldValidRule(
                "menuTitle".to_string(),
                ValidRule(ValidType::NotEmpty, "菜单名称不能为空!".to_string()),
            ),
            FieldValidRule(
                "menuType".to_string(),
                ValidRule(ValidType::NotEmpty, r#"菜单类型不能为空!"#.to_string()),
            ),
        ];
        valid_field_rule_slice(params, valid_rule.as_slice())
    }

    /// 更新菜单状态
    pub async fn update_menu_status(id: &String, status: &String) -> BmbpResp<usize> {
        if let Some(menu) = Self::find_menu_by_id(id).await? {
            if is_empty_prop(&menu, "menuCodePath") {
                return Err(BmbpError::service(
                    format!("异常的的菜单数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            if is_empty_prop(&menu, "menuCode") {
                return Err(BmbpError::service(
                    format!("异常的的菜单数据:{}，请联系管理员！", id).as_str(),
                ));
            }
            tracing::info!("menu:{:#?}", menu.get("menuCode").unwrap().clone());
            let mut script = MenuScript::update_status_script();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordStatus".to_string(), BmbpValue::from(status));
            match status.as_str() {
                "-1" => {
                    //停用 向下停用
                    script.filter("menu_code_path like concat(#{menuCodePath}::TEXT,'%')");
                    script_params.insert(
                        "menuCodePath".to_string(),
                        menu.get("menuCodePath").unwrap().clone(),
                    );
                }
                _ => {
                    let menu_code_path = menu.get("menuCodePath").unwrap().to_string();
                    let menu_code_array: Vec<String> =
                        menu_code_path.split("/").map(|x| x.to_string()).collect();
                    let trim_menu_code = menu_code_array
                        .iter()
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    script.filter("menu_code = ANY(#{menuCode})");
                    script_params.insert("menuCode".to_string(), BmbpValue::from(trim_menu_code));
                }
            }

            MenuDao::update(&script.to_script(), &script_params).await
        } else {
            return Err(BmbpError::service(
                format!("请定的菜单无效:{}", id).as_str(),
            ));
        }
    }
    /// 更新菜单上级
    pub async fn update_menu_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        let current_menu = {
            if let Some(menu) = Self::find_menu_by_id(record_id).await? {
                menu
            } else {
                return Err(BmbpError::service("待变更的菜单节点不存在"));
            }
        };

        let new_parent_menu = {
            if let Some(menu) = Self::find_menu_by_menu_code(parent).await? {
                menu
            } else {
                return Err(BmbpError::service("指定的上级菜单节点不存在"));
            }
        };
        // 当前节点属性
        let current_menu_code = current_menu.get("menuCode").unwrap().to_string();
        let current_menu_title = current_menu.get("menuTitle").unwrap().to_string();
        let current_menu_code_path = current_menu.get("menuCodePath").unwrap().to_string();
        let current_menu_title_path = current_menu.get("menuTitlePath").unwrap().to_string();
        // 计算原父点属性
        let menu_title_length = format!("{}/", current_menu_title).len();
        let menu_code_length = format!("{}/", current_menu_code).len();
        let current_menu_code_path_length = current_menu_code_path.len();
        let current_menu_title_path_length = current_menu_title_path.len();
        let old_parent_menu_cdoe_path =
            &current_menu_code_path[0..(current_menu_code_path_length - menu_code_length)];
        let old_parent_menu_title_path =
            &current_menu_title_path[0..(current_menu_title_path_length - menu_title_length)];
        let parent_menu_code_path = new_parent_menu.get("menuCodePath").unwrap().to_string();
        let parent_menu_title_path = new_parent_menu.get("menuTitlePath").unwrap().to_string();
        /// 还没有事务
        // 更新menuPathentCode
        let update_parent_script = MenuScript::update_parent_script();
        let mut update_parent_script_params = BmbpHashMap::new();
        update_parent_script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        update_parent_script_params.insert("menuParentCode".to_string(), BmbpValue::from(parent));

        // 更新自身及下级标题路径
        let mut update_menu_title_path_script = MenuScript::update_title_path_script();
        let mut update_menu_title_path_script_params = BmbpHashMap::new();
        update_menu_title_path_script_params.insert(
            "newMenuTitlePath".to_string(),
            BmbpValue::from(parent_menu_title_path),
        );
        update_menu_title_path_script_params.insert(
            "oldMenuParentTitlePath".to_string(),
            BmbpValue::from(old_parent_menu_title_path),
        );
        update_menu_title_path_script_params.insert(
            "currentMenuTitlePath".to_string(),
            BmbpValue::from(current_menu_title_path),
        );

        // 更新自身及下级编码路径
        let mut update_menu_code_path_script = MenuScript::update_code_path_script();

        let mut update_menu_code_path_script_params = BmbpHashMap::new();
        update_menu_code_path_script_params.insert(
            "newMenuCodePath".to_string(),
            BmbpValue::from(parent_menu_code_path),
        );
        update_menu_code_path_script_params.insert(
            "oldMenuParentCodePath".to_string(),
            BmbpValue::from(old_parent_menu_cdoe_path),
        );
        update_menu_code_path_script_params.insert(
            "currentMenuCodePath".to_string(),
            BmbpValue::from(current_menu_code_path),
        );
        let mut row_count = MenuDao::update(
            &update_parent_script.to_script(),
            &update_parent_script_params,
        )
        .await?;
        row_count = row_count
            + MenuDao::update(
                &update_menu_title_path_script.to_script(),
                &update_menu_title_path_script_params,
            )
            .await?;
        row_count = row_count
            + MenuDao::update(
                &update_menu_code_path_script.to_script(),
                &update_menu_code_path_script_params,
            )
            .await?;
        Ok(row_count)
    }
    /// 删除菜单
    pub async fn remove_menu_by_id(id: &String) -> BmbpResp<usize> {
        if let Some(menu) = Self::find_menu_by_id(id).await? {
            if Self::current_menu_has_children(&menu).await? {
                return Err(BmbpError::service("指定的菜单存在下级节点，无法删除"));
            }

            if Self::currrent_menu_has_user(&menu).await? {
                return Err(BmbpError::service("指定的菜单存在用户，无法删除"));
            }

            if Self::current_menu_has_data(&menu).await? {
                return Err(BmbpError::service("指定的菜单存在业务数据，无法删除"));
            }
            let script = MenuScript::delete_script_by_id();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordId".to_string(), BmbpValue::from(id));
            MenuDao::delete(&script.to_script(), &script_params).await
        } else {
            Err(BmbpError::service("指定的菜单不存在，无法删除!"))
        }
    }
    /// 删除菜单
    pub async fn remove_menu(params: &BmbpHashMap) -> BmbpResp<usize> {
        Err(BmbpError::service("服务未实现"))
    }
}

/// 校验逻辑
impl MenuService {
    /// 判断是否包含相同的数据关联
    pub async fn has_same_menu_code_menu(_menu: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    // 判断是否包含相同的数据关联
    pub async fn has_same_record_id_menu(_menu: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }

    /// 判断是否包含相同的数据关联
    pub async fn has_same_data_id_menu(_menu: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否包含相同菜单
    pub async fn has_same_title_menu(menu: &mut BmbpHashMap, is_update: bool) -> BmbpResp<bool> {
        if is_empty_prop(menu, "menuTitle") {
            return Err(BmbpError::service("菜单名称不允许为空"));
        }
        if is_empty_prop(menu, "menuParentCode") {
            return Err(BmbpError::service("菜单上级不允许为空"));
        }

        let mut script = MenuScript::query_script();
        script.filter("menu_title = #{menuTitle}");
        script.filter("menu_parent_code = #{menuParentCode}");
        if is_update {
            if is_empty_prop(menu, "recordId") {
                return Err(BmbpError::service("请指定待更新的菜单标识"));
            }
            script.filter("record_id != #{recordId}");
        }
        let menu_rs = MenuDao::find_menu_info(&script.to_script(), &menu).await?;
        Ok(menu_rs.is_some())
    }
    /// 判断是否包含下级
    pub async fn current_menu_has_children(_menu: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否关联用户
    pub async fn currrent_menu_has_user(_menu: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否关联业务
    pub async fn current_menu_has_data(_menu: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
}
