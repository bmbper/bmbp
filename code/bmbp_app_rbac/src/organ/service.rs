use bmbp_app_common::{
    set_insert_mode, BmbpAuthInfo, BmbpError, BmbpHashMap, BmbpResp, BmbpValue, PageParams, PageVo,
    ROOT_TREE_NODE,
};
use bmbp_app_utils::{simple_uuid_upper, TreeBuilder};
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
            return Err(BmbpError::api("指定的节点不存在"));
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
            return Err(BmbpError::api("指定的节点数据异常,请联系管理员"));
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
        params: &mut OrganQueryParam,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        params.set_organ_parent_code(parent.to_string());
        Self::find_organ_list(params).await
    }
    /// 查询组织详情-> 多个参数
    pub async fn find_organ_one(organ: &BmbpRbacOrgan) -> BmbpResp<Option<BmbpRbacOrgan>> {
        if let Some(record_id) = organ.get_base_model().get_record_id() {
            if !record_id.is_empty() {
                return Self::find_organ_by_id(&record_id).await;
            }
        }

        if let Some(organ_code) = organ.get_organ_code() {
            if !organ_code.is_empty() {
                return Self::find_organ_by_organ_code(&organ_code).await;
            }
        }
        Ok(None)
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
    pub async fn find_organ_by_organ_code(organ_code: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = OrganScript::query_script();
        script_param.insert("organ_code".to_string(), BmbpValue::from(organ_code));
        query_script.filter("organ_code = #{organ_code}");
        OrganDao::find_organ_info(&query_script.to_script(), &script_param).await
    }
    /// 查询组织详情-通过ORGAN_DATA_ID
    pub async fn find_organ_by_organ_data_id(
        organ_data_id: &String,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = OrganScript::query_script();
        script_param.insert("organ_data_id".to_string(), BmbpValue::from(organ_data_id));
        query_script.filter("organ_data_id = #{organ_data_id}");
        OrganDao::find_organ_info(&query_script.to_script(), &script_param).await
    }
    /// 保存组织
    pub async fn save_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        if let Some(record_id) = organ.get_base_model().get_record_id() {
            if !record_id.is_empty() {
                return Self::update_organ(organ).await;
            }
        }
        Self::insert_organ(organ).await
    }
    /// 新增组织
    pub async fn insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Self::build_insert_default_value(organ);
        let _: bool = Self::valid_insert_organ(organ)?;
        let _: bool = Self::check_same_organ_record_id(organ).await?;
        let _: bool = Self::check_same_organ_organ_code(organ).await?;
        let _: bool = Self::check_same_organ_data_id(organ).await?;
        let _: bool = Self::check_same_organ_title(organ).await?;
        let mut bmbp_hash_map = BmbpHashMap::from(organ);
        let script_insert = OrganScript::insert_script();
        OrganDao::insert(&script_insert.to_script(), &mut bmbp_hash_map).await
    }

    fn build_insert_default_value(organ: &mut BmbpRbacOrgan) {
        /// 设置新增时的默认值
        set_insert_mode(organ.get_mut_base_model(), &mut BmbpAuthInfo::default());

        if organ.get_organ_code().is_none() || organ.get_organ_code().unwrap().is_empty() {
            organ.set_organ_code(simple_uuid_upper());
        }

        if organ.get_organ_parent_code().is_none()
            || organ.get_organ_parent_code().unwrap().is_empty()
        {
            organ.set_organ_parent_code(ROOT_TREE_NODE.to_string());
        }
    }
    /// 更新组织,仅允许更新组织名称
    pub async fn update_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        if organ.get_organ_title().is_none() || organ.get_organ_title().unwrap().is_empty() {
            return Err(BmbpError::api("组织名称不能为空"));
        }
        let _: bool = Self::check_same_organ_title(organ).await?;

        let old_organ = {
            if let Some(record_id) = organ.get_base_model().get_record_id() {
                Self::find_organ_by_id(record_id).await?
            } else if let Some(organ_code) = organ.get_organ_code() {
                Self::find_organ_by_organ_code(organ_code).await?
            } else if let Some(organ_data_id) = organ.get_organ_data_id() {
                Self::find_organ_by_organ_data_id(organ_data_id).await?
            } else {
                None
            }
        };

        if old_organ.is_none() {
            return Err(BmbpError::api("指定的组织机构不存在，无法更新"));
        }

        let old_organ_info = old_organ.unwrap();
        /// 组织机构名称
        if old_organ_info
            .get_organ_title()
            .unwrap()
            .eq(organ.get_organ_title().unwrap())
        {
            return Ok(0);
        }

        let record_id = old_organ_info
            .get_base_model()
            .get_record_id()
            .unwrap()
            .clone();
        organ.get_mut_base_model().set_record_id(record_id);

        let mut update_script = OrganScript::update_status_script();
        update_script.filter("record_id=#{record_id}");
        let mut script_params = BmbpHashMap::from(organ);
        OrganDao::update(&update_script.to_script(), &script_params).await
    }

    /// 更新组织状态
    pub async fn update_organ_status(id: String, status: String) -> BmbpResp<usize> {
        let script_sql = OrganScript::update_status_script();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("record_id".to_string(), BmbpValue::from(id));
        script_params.insert("record_status".to_string(), BmbpValue::from(status));
        OrganDao::update(&script_sql.to_script(), &script_params).await
    }
    /// 更新组织上级
    pub async fn update_organ_parent_by_record_id(
        record_id: &String,
        parent: &String,
    ) -> BmbpResp<usize> {
        let organ_op = Self::find_organ_by_id(record_id).await?;
        if organ_op.is_none() {
            return Err(BmbpError::api("指定的组织机构不存在"));
        }

        let organ_parent_op = Self::find_organ_by_organ_code(record_id).await?;
        if organ_parent_op.is_none() {
            return Err(BmbpError::api("指定的上级组织机构不存在"));
        }

        let script_sql = OrganScript::update_parent_script();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("record_id".to_string(), BmbpValue::from(record_id));
        script_params.insert("organ_parent_code".to_string(), BmbpValue::from(parent));
        OrganDao::update(&script_sql.to_script(), &script_params).await
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
    pub async fn remove_organ(params: &OrganQueryParam) -> BmbpResp<usize> {
        let mut script = OrganScript::delete_script();
        let mut script_params = BmbpHashMap::new();
        if let Some(record_id) = params.get_record_id() {
            script.filter("record_id=#{record_id}");
            script_params.insert("record_id".to_string(), BmbpValue::from(record_id));
        }

        if let Some(organ_parent_code) = params.get_organ_parent_code() {
            script.filter("organ_parent_code=#{organ_parent_code}");
            script_params.insert(
                "organ_parent_code".to_string(),
                BmbpValue::from(organ_parent_code),
            );
        }

        if let Some(organ_title) = params.get_organ_title() {
            script.filter("organ_title=#{organ_title}");
            script_params.insert("organ_title".to_string(), BmbpValue::from(organ_title));
        }

        OrganDao::delete(&script.to_script(), &script_params).await
    }
}

/// 校验逻辑
#[allow(dead_code)]
#[allow(unused)]
impl OrganService {
    /// 保存时的数据校验
    pub fn valid_insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if organ.get_base_model().get_record_id().is_none()
            || organ.get_base_model().get_record_id().unwrap().is_empty()
        {
            return Err(BmbpError::api("组织主键不能为空"));
        }

        if organ.get_organ_title().is_none() || organ.get_organ_title().unwrap().is_empty() {
            return Err(BmbpError::api("组织名称不能为空"));
        }

        if organ.get_organ_code().is_none() || organ.get_organ_code().unwrap().is_empty() {
            return Err(BmbpError::api("组织编码不能为空"));
        }

        if organ.get_organ_parent_code().is_none()
            || organ.get_organ_parent_code().unwrap().is_empty()
        {
            return Err(BmbpError::api("组织上级不能为空"));
        }
        Ok(true)
    }
    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_organ_code(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if let Some(organ_code) = organ.get_organ_code() {
            let organ_info_op = Self::find_organ_by_organ_code(organ_code).await?;
            match organ_info_op {
                Some(_) => Err(BmbpError::api("存在相同编码的组织机构")),
                None => Ok(true),
            }
        } else {
            Err(BmbpError::api("组织机构编码不允许为空"))
        }
    }
    // 判断是否包含相同的数据关联
    pub async fn check_same_organ_record_id(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if let Some(record_id) = organ.get_base_model().get_record_id() {
            let organ_info_op = Self::find_organ_by_id(record_id).await?;
            match organ_info_op {
                Some(_) => Err(BmbpError::api("存在相同主键的组织机构")),
                None => Ok(true),
            }
        } else {
            Err(BmbpError::api("组织机构主键不允许为空"))
        }
    }

    /// 判断是否包含相同的数据关联
    pub async fn check_same_organ_data_id(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if let Some(organ_data_id) = organ.get_organ_data_id() {
            let organ_info_op = Self::find_organ_by_organ_data_id(organ_data_id).await?;
            match organ_info_op {
                Some(_) => Err(BmbpError::api("存在相同数据主键的组织机构")),
                None => Ok(true),
            }
        } else {
            Err(BmbpError::api("组织机构数据主键不允许为空"))
        }
    }
    /// 判断是否包含相同组织
    pub async fn check_same_organ_title(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        let mut script_sql = OrganScript::query_script();
        let mut script_params = BmbpHashMap::new();
        if let Some(organ_parent_code) = organ.get_organ_parent_code() {
            script_sql.filter("organ_parent_code = #{organ_parent_code}");
            script_params.insert(
                "organ_parent_code".to_string(),
                BmbpValue::from(organ_parent_code),
            );
        }

        if let Some(organ_title) = organ.get_organ_title() {
            script_sql.filter("organ_title = #{organ_title}");
            script_params.insert("organ_title".to_string(), BmbpValue::from(organ_title));
        }

        if let Some(record_id) = organ.get_base_model().get_record_id() {
            script_sql.filter("record_id !=# {record_id}");
            script_params.insert("record_id".to_string(), BmbpValue::from(record_id));
        }

        let organ_info_op =
            OrganDao::find_organ_info(&script_sql.to_script(), &script_params).await?;
        match organ_info_op {
            Some(_) => Err(BmbpError::api("同一层级下存在相同名称的组织机构")),
            None => Ok(true),
        }
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
