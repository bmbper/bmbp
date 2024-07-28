use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_orm::{
    simple_column, value_column, RdbcColumn, RdbcFilter, RdbcModel, RdbcTable, RdbcTableInner,
    RdbcTree, RdbcTreeUtil, UpdateWrapper, RDBC_DATA_UPDATE_TIME, RDBC_DATA_UPDATE_USER,
    RDBC_DISABLE, RDBC_ENABLE, RDBC_TREE_CODE_PATH, RDBC_TREE_NAME, RDBC_TREE_NAME_PATH,
    RDBC_TREE_PARENT_CODE, RDBC_TREE_ROOT_NODE,
};
use bmbp_app_utils::{is_empty_string, simple_uuid_upper};

use crate::organ::dao::BmbpRbacOrganDao;
use crate::organ::model::{BmbpRbacOrgan, BmbpRbacOrganTree, BmbpRdbcOrganType, OrganQueryParams};
use crate::organ::script::BmbpRbacOrganScript;

pub struct BmbpRbacOrganService;

impl BmbpRbacOrganService {
    pub async fn find_organ_tree(
        params: OrganQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        if let Some(organ_list) = Self::find_organ_list(params).await? {
            let organ_tree = RdbcTreeUtil::build_tree(organ_list);
            return Ok(Some(organ_tree));
        }
        Ok(None)
    }
    pub async fn find_organ_tree_by_code(
        code: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        let mut params = OrganQueryParams::default();
        params.set_code_option(code);
        if let Some(organ_list) = Self::find_organ_list(params).await? {
            let organ_tree = RdbcTreeUtil::build_tree(organ_list);
            return Ok(Some(organ_tree));
        }
        Ok(None)
    }
    pub async fn find_organ_tree_by_id(
        id: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        let mut params = OrganQueryParams::default();
        params.set_data_id_option(id);
        if let Some(organ_list) = Self::find_organ_list(params).await? {
            let organ_tree = RdbcTreeUtil::build_tree(organ_list);
            return Ok(Some(organ_tree));
        }
        Ok(None)
    }
    pub async fn find_organ_tree_exclude_by_person(
        params: OrganQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        if let Some(organ_list) = Self::find_organ_list_exclude_by_person(params).await? {
            let organ_tree = RdbcTreeUtil::build_tree(organ_list);
            return Ok(Some(organ_tree));
        }
        Ok(None)
    }

    pub async fn find_organ_list(
        params: OrganQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        let mut query = BmbpRbacOrganScript::build_query_script();
        if let Some(parent) = params.get_parent_code() {
            query.eq_("parent_code", parent);
        }
        BmbpRbacOrganDao::select_list_by_query(&query).await
    }

    pub async fn find_organ_list_exclude_by_person(
        _params: OrganQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        let mut query = BmbpRbacOrganScript::build_query_script();
        query.ne_("organ_type", BmbpRdbcOrganType::PERSON.value());
        BmbpRbacOrganDao::select_list_by_query(&query).await
    }

    pub async fn find_organ_page(
        params: BmbpPageParam<OrganQueryParams>,
    ) -> BmbpResp<PageVo<BmbpRbacOrganTree>> {
        // 拼接查询条件
        let mut query = BmbpRbacOrganScript::build_query_script();
        if let Some(organ_params) = params.get_params() {
            let parent_code = match organ_params.get_parent_code() {
                None => RDBC_TREE_ROOT_NODE.to_string(),
                Some(v) => v.to_string(),
            };
            query.eq_("parent_code", parent_code);
        };

        BmbpRbacOrganDao::select_page_by_query(params.get_page_no(), params.get_page_size(), &query)
            .await
    }

    pub async fn find_organ_by_id(id: Option<&String>) -> BmbpResp<Option<BmbpRbacOrganTree>> {
        if is_empty_string(id.as_ref()) {
            return Ok(None);
        }
        let mut query = BmbpRbacOrganScript::build_query_script();
        query.eq_(BmbpRbacOrgan::get_table_primary_key(), id.unwrap());
        BmbpRbacOrganDao::select_one_by_query(&query).await
    }
    async fn find_organ_by_code(code: Option<&String>) -> BmbpResp<Option<BmbpRbacOrganTree>> {
        let mut query = BmbpRbacOrganScript::build_query_script();
        query.eq_("code", code);
        BmbpRbacOrganDao::select_one_by_query(&query).await
    }
    pub async fn query_organ_tree_exclude_by_id(
        organ_id: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        if let Some(organ_list) = Self::query_organ_list_exclude_by_id(organ_id).await? {
            let organ_tree = RdbcTreeUtil::build_tree(organ_list);
            return Ok(Some(organ_tree));
        }
        Ok(None)
    }
    pub async fn query_organ_list_exclude_by_id(
        organ_id: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrganTree>>> {
        let organ_info = Self::find_organ_by_id(organ_id.as_ref()).await?;
        if organ_info.is_none() {
            return Err(BmbpError::service("指定的组织机构不存在!"));
        }
        let organ_info = organ_info.unwrap();
        let organ_code_path = organ_info.get_code_path();
        if organ_code_path.is_none() {
            return Err(BmbpError::service("组织机构数据不准确，请联系管理员!"));
        }
        let code_path = organ_code_path.unwrap().clone();
        let mut query = BmbpRbacOrganScript::build_query_script();
        query.not_like_left("code_path", code_path);
        BmbpRbacOrganDao::select_list_by_query(&query).await
    }
    pub async fn insert_organ(organ: &mut BmbpRbacOrganTree) -> BmbpResp<usize> {
        // 设置公共默认值
        organ.init_values();
        if is_empty_string(organ.get_name()) {
            return Err(BmbpError::service("组织机构名称不能为空"));
        }
        // 设置组织机构编码默认值
        organ.set_data_id(simple_uuid_upper());
        organ.set_code(simple_uuid_upper());

        // 根据上级节点设置本级的编码路径、本级的名称路径
        // 当上级节点编码为空时，赋值为根节点
        // 根据上级编码去查询节点信息，查询不到节点时，
        // 当前节点编码为根节点，设置根路径， 不为根节点时，提示找不到指定的上级节点
        // 计算本级节点编码路径、本级节点名称路径
        if is_empty_string(organ.get_parent_code().as_ref()) {
            organ.set_parent_code(RDBC_TREE_ROOT_NODE.to_string());
        }
        let mut parent_code_path = "".to_string();
        let mut parent_name_path = "".to_string();
        if let Some(organ_parent) = Self::find_organ_by_code(organ.get_parent_code()).await? {
            if let Some(code_path_tmp) = organ_parent.get_code_path() {
                parent_code_path = format!("{}", code_path_tmp);
            }
            if let Some(name_path_tmp) = organ_parent.get_name_path() {
                parent_name_path = format!("{}", name_path_tmp);
            }
        } else {
            if organ.get_parent_code().unwrap() != RDBC_TREE_ROOT_NODE {
                return Err(BmbpError::service("找不到指定的上级节点!"));
            }
            parent_code_path = "/".to_string();
            parent_name_path = "/".to_string();
        }

        let code_path = format!("{}{}/", parent_code_path, organ.get_code().unwrap());
        let name_path = format!("{}{}/", parent_name_path, organ.get_name().unwrap());
        organ.set_code_path(code_path.clone());
        organ.set_name_path(name_path.clone());
        // 计算树的节点层级
        organ.set_node_leaf(1);
        let node_levels = code_path.split("/").count() - 2;
        organ.set_node_level(node_levels);
        if Self::has_same_name(
            organ.get_name().unwrap(),
            organ.get_parent_code(),
            organ.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("组织机构名称已被占用，请修改后重试"));
        }
        let insert = BmbpRbacOrganScript::build_insert(&organ);
        BmbpRbacOrganDao::execute_insert(&insert).await
    }

    pub async fn update_organ(organ: &mut BmbpRbacOrganTree) -> BmbpResp<usize> {
        let old_organ_op = Self::find_organ_by_id(organ.get_data_id()).await?;
        if old_organ_op.is_none() {
            return Err(BmbpError::service("指定的组织机构不存在!"));
        }
        let old_organ = old_organ_op.unwrap();
        {
            if is_empty_string(organ.get_name()) {
                if let Some(old_name) = old_organ.get_name() {
                    organ.set_name(old_name.to_string());
                }
            }
            if is_empty_string(organ.get_code()) {
                if let Some(old_code) = old_organ.get_code() {
                    organ.set_code(old_code.to_string());
                }
            }
            if !organ.get_data_remark().is_none() {
                if let Some(old_data_remark) = old_organ.get_data_remark() {
                    organ.set_data_remark(old_data_remark.to_string());
                }
            }
            if organ.get_data_sort().is_none() {
                if let Some(old_organ_sort) = old_organ.get_data_sort() {
                    organ.set_data_sort(old_organ_sort.clone());
                }
            }

            let old_ext_organ = old_organ.get_ext_props();
            let ext_organ_props = organ.get_ext_props_mut();

            if is_empty_string(ext_organ_props.get_organ_type()) {
                if let Some(old_organ_type) = old_ext_organ.get_organ_type() {
                    ext_organ_props.set_organ_type(old_organ_type.clone());
                }
            }
        }

        // 取上级节点
        let mut parent_organ_code = old_organ.get_parent_code();
        if is_empty_string(parent_organ_code) {
            parent_organ_code = organ.get_parent_code();
        }
        if is_empty_string(parent_organ_code) {
            return Err(BmbpError::service(
                "指定的组织机构数据缺少上级组织机构，请联系管理员修复！",
            ));
        }
        let mut parent_code_path = "/".to_string();
        let mut parent_name_path = "/".to_string();
        let parent_organ_op = Self::find_organ_by_code(parent_organ_code).await?;
        if let Some(parent) = parent_organ_op {
            if let Some(code_path_tmp) = parent.get_code_path() {
                parent_code_path = format!("{}", code_path_tmp);
            }
            if let Some(name_path_tmp) = parent.get_name_path() {
                parent_name_path = format!("{}", name_path_tmp);
            }
        } else {
            if parent_organ_code.unwrap() != RDBC_TREE_ROOT_NODE {
                return Err(BmbpError::service("找不到指定的上级节点!"));
            }
        }

        organ.set_code_path(format!(
            "{}{}/",
            parent_code_path,
            old_organ.get_code().unwrap()
        ));
        organ.set_name_path(format!(
            "{}{}/",
            parent_name_path,
            organ.get_name().unwrap()
        ));
        let ext_organ_props = organ.get_ext_props();
        if is_empty_string(ext_organ_props.get_organ_type()) {
            return Err(BmbpError::service("组织机构类型不允许为空"));
        }

        if is_empty_string(organ.get_name()) {
            return Err(BmbpError::service("组织机构名称不允许为空"));
        }
        if is_empty_string(organ.get_code()) {
            return Err(BmbpError::service("组织机构编码不允许为空"));
        }

        if Self::has_same_name(
            organ.get_name().unwrap(),
            organ.get_parent_code(),
            organ.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("组织机构名称已被占用，请修改后重试"));
        }
        let update = BmbpRbacOrganScript::build_update(organ);
        let mut row_count = BmbpRbacOrganDao::execute_update(&update).await?;
        // 更新子表数据
        row_count = row_count
            + Self::update_name_path_for_children(old_organ.get_name_path().unwrap()).await?;
        Ok(row_count)
    }
    pub async fn update_organ_parent(
        organ_id: Option<String>,
        parent_code: Option<String>,
    ) -> BmbpResp<usize> {
        let organ_info = Self::find_organ_by_id(organ_id.as_ref()).await?;
        if organ_info.is_none() {
            return Err(BmbpError::service("指定的组织机构不存在!"));
        }
        let mut organ = organ_info.unwrap();
        let old_code_path = organ.get_code_path().unwrap().clone();
        let old_name_path = organ.get_name_path().unwrap().clone();
        if parent_code.is_none() {
            return Err(BmbpError::service("请指定变更的目标组织机构!"));
        }
        let parent_code = parent_code.unwrap().clone();
        let mut parent_code_path = "/".to_string();
        let mut parent_name_path = "/".to_string();
        if RDBC_TREE_ROOT_NODE == parent_code {
        } else {
            let parent_organ = Self::find_organ_by_code(Some(&parent_code)).await?;
            if parent_organ.is_none() {
                return Err(BmbpError::service("请指定变更的目标组织机构!"));
            }
            let parent_organ = parent_organ.unwrap();
            parent_code_path = parent_organ.get_code_path().unwrap().clone();
            parent_name_path = parent_organ.get_name_path().unwrap().clone();
        }
        organ.set_parent_code(parent_code.clone());
        organ.set_code_path(format!(
            "{}{}/",
            parent_code_path,
            organ.get_code().unwrap()
        ));
        organ.set_name_path(format!(
            "{}{}/",
            parent_name_path,
            organ.get_name().unwrap()
        ));
        if Self::has_same_name(
            organ.get_name().unwrap(),
            organ.get_parent_code(),
            organ.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service(
                "目标组织机构下已存在相同名称组织机构，无法变更",
            ));
        }
        organ.init_update_values();
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpRbacOrgan::get_table_name())
            .set(RDBC_TREE_PARENT_CODE, organ.get_parent_code())
            .set(RDBC_TREE_CODE_PATH, organ.get_code_path())
            .set(RDBC_TREE_NAME_PATH, organ.get_name_path())
            .set(RDBC_DATA_UPDATE_TIME, organ.get_data_update_time())
            .set(RDBC_DATA_UPDATE_USER, organ.get_data_update_user())
            .eq_(
                BmbpRbacOrgan::get_table_primary_key(),
                organ.get_data_id().unwrap(),
            );
        let mut row_count = BmbpRbacOrganDao::execute_update(&update).await?;
        row_count += Self::update_code_path_for_children(old_code_path.as_str()).await?;
        let _ = Self::update_name_path_for_children(old_name_path.as_str()).await?;
        Ok(row_count)
    }

    async fn update_name_path_for_children(parent_name_path: &str) -> BmbpResp<usize> {
        let mut update = UpdateWrapper::new();
        update.table_alias(BmbpRbacOrgan::get_table_name(), "t1".to_string());
        let mut join_table =
            RdbcTableInner::table_alias(BmbpRbacOrgan::get_table_name(), "t2".to_string());
        join_table.on_eq_col(
            RdbcColumn::table_column("t1", "parent_code"),
            RdbcColumn::table_column("t2", "code"),
        );
        update.join_rdbc_table(join_table);

        let concat_column = RdbcColumn::concat(vec![
            simple_column("t2", "name_path"),
            simple_column("t1", "name"),
            value_column("/"),
        ]);
        update.set("name_path", concat_column);
        update.like_left_value(simple_column("t1", "name_path"), parent_name_path);
        BmbpRbacOrganDao::execute_update(&update).await
    }
    async fn update_code_path_for_children(parent_code_path: &str) -> BmbpResp<usize> {
        let mut update = UpdateWrapper::new();
        update.table_alias(BmbpRbacOrgan::get_table_name(), "t1".to_string());
        let concat_column = RdbcColumn::concat(vec![
            simple_column("t2", "code_path"),
            simple_column("t1", "code"),
            value_column("/"),
        ]);
        update.set(simple_column("t1", "code_path"), concat_column);
        let mut join_table =
            RdbcTableInner::table_alias(BmbpRbacOrgan::get_table_name(), "t2".to_string());
        join_table.on_eq_col(
            RdbcColumn::table_column("t1", "parent_code"),
            RdbcColumn::table_column("t2", "code"),
        );
        update.join_rdbc_table(join_table);
        update.like_left_value(simple_column("t1", "code_path"), parent_code_path);
        BmbpRbacOrganDao::execute_update(&update).await
    }
    pub async fn disable_organ(organ_id: Option<String>) -> BmbpResp<usize> {
        let organ = Self::find_organ_by_id(organ_id.as_ref()).await?;
        if organ.is_none() {
            return Err(BmbpError::service("待停用的组织机构不存在!"));
        }
        let organ_info = organ.unwrap();
        let code_path = organ_info.get_code_path().unwrap();
        let update = BmbpRbacOrganScript::build_update_status(code_path, RDBC_DISABLE);
        BmbpRbacOrganDao::execute_update(&update).await
    }

    pub async fn enable_organ(organ_id: Option<String>) -> BmbpResp<usize> {
        let organ = Self::find_organ_by_id(organ_id.as_ref()).await?;
        if organ.is_none() {
            return Err(BmbpError::service("待启用的组织机构不存在!"));
        }
        let organ_info = organ.unwrap();
        let code_path = organ_info.get_code_path().unwrap();
        let update = BmbpRbacOrganScript::build_update_status(code_path, RDBC_ENABLE);
        BmbpRbacOrganDao::execute_update(&update).await
    }

    pub async fn delete_organ_by_id(organ_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(organ_id.as_ref()) {
            return Err(BmbpError::service("请指定待删除的组织机构!"));
        }
        let delete_organ = BmbpRbacOrganScript::build_delete_script(organ_id);
        BmbpRbacOrganDao::execute_delete(&delete_organ).await
    }

    pub async fn has_same_name(
        name: &String,
        parent_code: Option<&String>,
        data_id: Option<&String>,
    ) -> BmbpResp<bool> {
        let mut query = BmbpRbacOrganScript::build_query_script();
        query.eq_(RDBC_TREE_NAME, name);
        if parent_code.is_some() {
            query.eq_(RDBC_TREE_PARENT_CODE, parent_code.unwrap());
        }
        if data_id.is_some() {
            query.ne_(BmbpRbacOrgan::get_table_primary_key(), data_id.unwrap());
        }
        let organ_vec = BmbpRbacOrganDao::select_list_by_query(&query).await?;
        match organ_vec {
            Some(organ_vec) => Ok(!organ_vec.is_empty()),
            None => Ok(false),
        }
    }
}
