use std::collections::HashMap;
use std::process::id;

use serde_json::{Map, Value};
use tracing::info;

use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_utils::{is_empty_string, simple_uuid_upper};
use bmbp_rdbc_orm::{
    simple_column, value_column, QueryWrapper, RdbcColumn, RdbcFilter, RdbcModel, RdbcTable,
    RdbcTableInner, RdbcTree, RdbcTreeUtil, UpdateWrapper, RDBC_DATA_UPDATE_TIME,
    RDBC_DATA_UPDATE_USER, RDBC_DISABLE, RDBC_ENABLE, RDBC_TREE_CODE_PATH, RDBC_TREE_NAME,
    RDBC_TREE_NAME_PATH, RDBC_TREE_PARENT_CODE, RDBC_TREE_ROOT_NODE,
};

use crate::dict::dao::BmbpRbacDictDao;
use crate::dict::model::{
    BmbpComboVo, BmbpDictType, BmbpSettingDict, BmbpSettingDictOrmModel, DictQueryParams,
};
use crate::dict::scripts::BmbpRdbcDictScript;
use crate::dict::web::find_dict_info;

pub struct BmbpRbacDictService {}

impl BmbpRbacDictService {
    pub async fn find_dict_tree(
        params: DictQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        if let Some(dict_list) = Self::find_dict_list(params).await? {
            let dict_tree = RdbcTreeUtil::build_tree(dict_list);
            return Ok(Some(dict_tree));
        }
        Ok(None)
    }

    pub async fn find_dict_list(
        params: DictQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        if let Some(parent) = params.get_parent_code() {
            query.eq_("parent_code", parent);
        }
        BmbpRbacDictDao::select_list_by_query(&query).await
    }

    pub async fn find_dict_page(
        params: BmbpPageParam<DictQueryParams>,
    ) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
        // 拼接查询条件
        let mut query = BmbpRdbcDictScript::build_query_script();
        if let Some(dict_params) = params.get_params() {
            let parent_code = match dict_params.get_parent_code() {
                None => RDBC_TREE_ROOT_NODE.to_string(),
                Some(v) => v.to_string(),
            };
            query.eq_("parent_code", parent_code);
        };

        BmbpRbacDictDao::select_page_by_query(params.get_page_no(), params.get_page_size(), &query)
            .await
    }

    pub async fn find_dict_by_id(id: Option<&String>) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        if is_empty_string(id.as_ref()) {
            return Ok(None);
        }
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_(BmbpSettingDict::get_table_primary_key(), id.unwrap());
        info!(
            "query_dict_by_id: id={}",
            BmbpSettingDict::get_table_primary_key()
        );
        BmbpRbacDictDao::select_one_by_query(&query).await
    }
    async fn find_dict_by_code(code: &String) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_("code", code);
        BmbpRbacDictDao::select_one_by_query(&query).await
    }
    pub async fn find_dict_by_alias(alias: &String) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_("dict_alias", alias);
        BmbpRbacDictDao::select_one_by_query(&query).await
    }
    pub async fn query_dict_tree_exclude_by_id(
        dict_id: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        if let Some(dict_list) = Self::query_dict_list_exclude_by_id(dict_id).await? {
            let dict_tree = RdbcTreeUtil::build_tree(dict_list);
            return Ok(Some(dict_tree));
        }
        Ok(None)
    }
    pub async fn query_dict_list_exclude_by_id(
        dict_id: Option<String>,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let dict_info = Self::find_dict_by_id(dict_id.as_ref()).await?;
        if dict_info.is_none() {
            return Err(BmbpError::service("指定的字典不存在!"));
        }
        let dict_info = dict_info.unwrap();
        let dict_code_path = dict_info.get_code_path();
        if dict_code_path.is_none() {
            return Err(BmbpError::service("字典数据不准确，请联系管理员!"));
        }
        let code_path = dict_code_path.unwrap().clone();
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.not_like_left("code_path", code_path);
        BmbpRbacDictDao::select_list_by_query(&query).await
    }
    pub async fn find_children_by_code(
        code: Option<&String>,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_(RDBC_TREE_PARENT_CODE, code);
        BmbpRbacDictDao::select_list_by_query(&query).await
    }
    pub async fn find_children_by_code_path(
        code_path: Option<&String>,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.like_left_value(RDBC_TREE_CODE_PATH, code_path);
        query.ne_(RDBC_TREE_CODE_PATH, code_path);
        BmbpRbacDictDao::select_list_by_query(&query).await
    }

    pub async fn insert_dict(dict: &mut BmbpSettingDictOrmModel) -> BmbpResp<usize> {
        // 设置公共默认值
        dict.init_values();
        if is_empty_string(dict.get_name()) {
            return Err(BmbpError::service("字典名称不能为空"));
        }
        if is_empty_string(dict.get_ext_props().get_dict_alias()) {
            return Err(BmbpError::service("字典别名不能为空"));
        }
        if is_empty_string(dict.get_ext_props().get_dict_value()) {
            return Err(BmbpError::service("字典值不能为空"));
        }
        // 设置字典编码默认值
        dict.set_data_id(simple_uuid_upper());
        dict.set_code(simple_uuid_upper());
        dict.get_ext_props_mut().set_dict_type(BmbpDictType::Custom);
        // 根据上级节点设置本级的编码路径、本级的名称路径
        // 当上级节点编码为空时，赋值为根节点
        // 根据上级编码去查询节点信息，查询不到节点时，
        // 当前节点编码为根节点，设置根路径， 不为根节点时，提示找不到指定的上级节点
        // 计算本级节点编码路径、本级节点名称路径
        if is_empty_string(dict.get_parent_code().as_ref()) {
            dict.set_parent_code(RDBC_TREE_ROOT_NODE.to_string());
        }
        let mut parent_code_path = "".to_string();
        let mut parent_name_path = "".to_string();
        if let Some(dict_parent) = Self::find_dict_by_code(dict.get_parent_code().unwrap()).await? {
            if let Some(code_path_tmp) = dict_parent.get_code_path() {
                parent_code_path = format!("{}", code_path_tmp);
            }
            if let Some(name_path_tmp) = dict_parent.get_name_path() {
                parent_name_path = format!("{}", name_path_tmp);
            }
        } else {
            if dict.get_parent_code().unwrap() != RDBC_TREE_ROOT_NODE {
                return Err(BmbpError::service("找不到指定的上级节点!"));
            }
            parent_code_path = "/".to_string();
            parent_name_path = "/".to_string();
        }

        let code_path = format!("{}{}/", parent_code_path, dict.get_code().unwrap());
        let name_path = format!("{}{}/", parent_name_path, dict.get_name().unwrap());
        dict.set_code_path(code_path.clone());
        dict.set_name_path(name_path.clone());
        // 计算树的节点层级
        dict.set_node_leaf(1);
        let node_levels = code_path.split("/").count() - 2;
        dict.set_node_level(node_levels);
        if Self::has_same_name(
            dict.get_name().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典名称已被占用，请修改后重试"));
        }
        if Self::has_same_alias(
            dict.get_ext_props().get_dict_alias().unwrap(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典别名已被占用，请修改后重试"));
        }
        if Self::has_same_value(
            dict.get_ext_props().get_dict_value().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典别名已被占用，请修改后重试"));
        }
        let insert = BmbpRdbcDictScript::build_insert(&dict);
        BmbpRbacDictDao::execute_insert(&insert).await
    }

    pub async fn update_dict(dict: &mut BmbpSettingDictOrmModel) -> BmbpResp<usize> {
        let old_dict_op = Self::find_dict_by_id(dict.get_data_id()).await?;
        if old_dict_op.is_none() {
            return Err(BmbpError::service("指定的字典不存在!"));
        }
        let old_dict = old_dict_op.unwrap();
        {
            if is_empty_string(dict.get_name()) {
                if let Some(old_name) = old_dict.get_name() {
                    dict.set_name(old_name.to_string());
                }
            }
            if is_empty_string(dict.get_code()) {
                if let Some(old_code) = old_dict.get_code() {
                    dict.set_code(old_code.to_string());
                }
            }
            if !dict.get_data_remark().is_none() {
                if let Some(old_data_remark) = old_dict.get_data_remark() {
                    dict.set_data_remark(old_data_remark.to_string());
                }
            }
            if dict.get_data_sort().is_none() {
                if let Some(old_dict_sort) = old_dict.get_data_sort() {
                    dict.set_data_sort(old_dict_sort.clone());
                }
            }

            let old_ext_dict = old_dict.get_ext_props();
            let ext_dict_props = dict.get_ext_props_mut();

            if is_empty_string(ext_dict_props.get_dict_value()) {
                if let Some(old_dict_value) = old_ext_dict.get_dict_value() {
                    ext_dict_props.set_dict_value(old_dict_value.to_string());
                }
            }

            if is_empty_string(ext_dict_props.get_dict_alias()) {
                if let Some(old_dict_alias) = old_ext_dict.get_dict_alias() {
                    ext_dict_props.set_dict_alias(old_dict_alias.to_string());
                }
            }

            if is_empty_string(ext_dict_props.get_dict_type()) {
                if let Some(old_dict_type) = old_ext_dict.get_dict_type() {
                    ext_dict_props.set_dict_type(old_dict_type.clone());
                }
            }
        }

        // 取上级节点
        let mut parent_dict_code = old_dict.get_parent_code();
        if is_empty_string(parent_dict_code) {
            parent_dict_code = dict.get_parent_code();
        }
        if is_empty_string(parent_dict_code) {
            return Err(BmbpError::service(
                "指定的字典数据缺少上级字典，请联系管理员修复！",
            ));
        }
        let mut parent_code_path = "/".to_string();
        let mut parent_name_path = "/".to_string();
        let parent_dict_op = Self::find_dict_by_code(parent_dict_code.unwrap()).await?;
        if let Some(parent) = parent_dict_op {
            if let Some(code_path_tmp) = parent.get_code_path() {
                parent_code_path = format!("{}", code_path_tmp);
            }
            if let Some(name_path_tmp) = parent.get_name_path() {
                parent_name_path = format!("{}", name_path_tmp);
            }
        } else {
            if parent_dict_code.unwrap() != RDBC_TREE_ROOT_NODE {
                return Err(BmbpError::service("找不到指定的上级节点!"));
            }
        }

        dict.set_code_path(format!(
            "{}{}/",
            parent_code_path,
            old_dict.get_code().unwrap()
        ));
        dict.set_name_path(format!("{}{}/", parent_name_path, dict.get_name().unwrap()));
        let ext_dict_props = dict.get_ext_props();
        if is_empty_string(ext_dict_props.get_dict_value()) {
            return Err(BmbpError::service("字典值不允许为空"));
        }
        if is_empty_string(ext_dict_props.get_dict_alias()) {
            return Err(BmbpError::service("字典别名不允许为空"));
        }
        if is_empty_string(ext_dict_props.get_dict_type()) {
            return Err(BmbpError::service("字典类型不允许为空"));
        }

        if is_empty_string(dict.get_name()) {
            return Err(BmbpError::service("字典名称不允许为空"));
        }
        if is_empty_string(dict.get_code()) {
            return Err(BmbpError::service("字典编码不允许为空"));
        }

        if Self::has_same_name(
            dict.get_name().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典名称已被占用，请修改后重试"));
        }
        if Self::has_same_alias(
            dict.get_ext_props().get_dict_alias().unwrap(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典别名已被占用，请修改后重试"));
        }
        if Self::has_same_value(
            dict.get_ext_props().get_dict_value().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("字典别名已被占用，请修改后重试"));
        }

        let update = BmbpRdbcDictScript::build_update(dict);
        let mut row_count = BmbpRbacDictDao::execute_update(&update).await?;
        // 更新子表数据
        row_count = row_count
            + Self::update_name_path_for_children(old_dict.get_name_path().unwrap()).await?;
        Ok(row_count)
    }
    pub async fn update_dict_parent(
        dict_id: Option<String>,
        parent_code: Option<String>,
    ) -> BmbpResp<usize> {
        let dict_info = Self::find_dict_by_id(dict_id.as_ref()).await?;
        if dict_info.is_none() {
            return Err(BmbpError::service("指定的字典不存在!"));
        }
        let mut dict = dict_info.unwrap();
        let old_code_path = dict.get_code_path().unwrap().clone();
        let old_name_path = dict.get_name_path().unwrap().clone();
        if parent_code.is_none() {
            return Err(BmbpError::service("请指定变更的目标字典!"));
        }
        let parent_code = parent_code.unwrap().clone();
        let mut parent_code_path = "".to_string();
        let mut parent_name_path = "".to_string();
        if RDBC_TREE_ROOT_NODE == parent_code {
            parent_code_path = "/".to_string();
            parent_name_path = "/".to_string();
        } else {
            let parent_dict = Self::find_dict_by_code(&parent_code).await?;
            if parent_dict.is_none() {
                return Err(BmbpError::service("请指定变更的目标字典!"));
            }
            let parent_dict = parent_dict.unwrap();
            parent_code_path = parent_dict.get_code_path().unwrap().clone();
            parent_name_path = parent_dict.get_name_path().unwrap().clone();
        }
        dict.set_parent_code(parent_code.clone());
        dict.set_code_path(format!("{}{}/", parent_code_path, dict.get_code().unwrap()));
        dict.set_name_path(format!("{}{}/", parent_name_path, dict.get_name().unwrap()));
        if Self::has_same_name(
            dict.get_name().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("目标字典下已存在相同名称字典，无法变更"));
        }
        if Self::has_same_alias(
            dict.get_ext_props().get_dict_alias().unwrap(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("目标字典下已存在相同别名字典，无法变更"));
        }
        if Self::has_same_value(
            dict.get_ext_props().get_dict_value().unwrap(),
            dict.get_parent_code(),
            dict.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service(
                "目标字典下已存在相同字典值的字典，无法变更",
            ));
        }
        dict.init_update_values();
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpSettingDict::get_table_name())
            .set(RDBC_TREE_PARENT_CODE, dict.get_parent_code())
            .set(RDBC_TREE_CODE_PATH, dict.get_code_path())
            .set(RDBC_TREE_NAME_PATH, dict.get_name_path())
            .set(RDBC_DATA_UPDATE_TIME, dict.get_data_update_time())
            .set(RDBC_DATA_UPDATE_USER, dict.get_data_update_user())
            .eq_(
                BmbpSettingDict::get_table_primary_key(),
                dict.get_data_id().unwrap(),
            );
        let mut row_count = BmbpRbacDictDao::execute_update(&update).await?;
        row_count += Self::update_code_path_for_children(old_code_path.as_str()).await?;
        let _ = Self::update_name_path_for_children(old_name_path.as_str()).await?;
        Ok(row_count)
    }

    async fn update_name_path_for_children(parent_name_path: &str) -> BmbpResp<usize> {
        let mut update = UpdateWrapper::new();
        update.table_alias(BmbpSettingDict::get_table_name(), "t1".to_string());
        let mut join_table =
            RdbcTableInner::table_alias(BmbpSettingDict::get_table_name(), "t2".to_string());
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
        BmbpRbacDictDao::execute_update(&update).await
    }
    async fn update_code_path_for_children(parent_code_path: &str) -> BmbpResp<usize> {
        let mut update = UpdateWrapper::new();
        update.table_alias(BmbpSettingDict::get_table_name(), "t1".to_string());
        let concat_column = RdbcColumn::concat(vec![
            simple_column("t2", "code_path"),
            simple_column("t1", "code"),
            value_column("/"),
        ]);
        update.set(simple_column("t1", "code_path"), concat_column);
        let mut join_table =
            RdbcTableInner::table_alias(BmbpSettingDict::get_table_name(), "t2".to_string());
        join_table.on_eq_col(
            RdbcColumn::table_column("t1", "parent_code"),
            RdbcColumn::table_column("t2", "code"),
        );
        update.join_rdbc_table(join_table);
        update.like_left_value(simple_column("t1", "code_path"), parent_code_path);
        BmbpRbacDictDao::execute_update(&update).await
    }
    pub async fn disable_dict(dict_id: Option<String>) -> BmbpResp<usize> {
        let mut dict = Self::find_dict_by_id(dict_id.as_ref()).await?;
        if dict.is_none() {
            return Err(BmbpError::service("待停用的字典不存在!"));
        }
        let dict_info = dict.unwrap();
        let code_path = dict_info.get_code_path().unwrap();
        let update = BmbpRdbcDictScript::build_update_status(code_path, RDBC_DISABLE);
        BmbpRbacDictDao::execute_update(&update).await
    }

    pub async fn enable_dict(dict_id: Option<String>) -> BmbpResp<usize> {
        let dict = Self::find_dict_by_id(dict_id.as_ref()).await?;
        if dict.is_none() {
            return Err(BmbpError::service("待启用的字典不存在!"));
        }
        let dict_info = dict.unwrap();
        let code_path = dict_info.get_code_path().unwrap();
        let update = BmbpRdbcDictScript::build_update_status(code_path, RDBC_ENABLE);
        BmbpRbacDictDao::execute_update(&update).await
    }

    pub async fn delete_dict_by_id(dict_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(dict_id.as_ref()) {
            return Err(BmbpError::service("请指定待删除的字典!"));
        }
        let delete_dict = BmbpRdbcDictScript::build_delete_script(dict_id);
        BmbpRbacDictDao::execute_delete(&delete_dict).await
    }

    pub async fn has_same_name(
        name: &String,
        parent_code: Option<&String>,
        data_id: Option<&String>,
    ) -> BmbpResp<bool> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_(RDBC_TREE_NAME, name);
        if parent_code.is_some() {
            query.eq_(RDBC_TREE_PARENT_CODE, parent_code.unwrap());
        }
        if data_id.is_some() {
            query.ne_(BmbpSettingDict::get_table_primary_key(), data_id.unwrap());
        }
        let dict_vec = BmbpRbacDictDao::select_list_by_query(&query).await?;
        match dict_vec {
            Some(dict_vec) => Ok(!dict_vec.is_empty()),
            None => Ok(false),
        }
    }
    pub async fn has_same_value(
        value: &String,
        parent_code: Option<&String>,
        data_id: Option<&String>,
    ) -> BmbpResp<bool> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_("dict_value", value);
        if parent_code.is_some() {
            query.eq_(RDBC_TREE_PARENT_CODE, parent_code.unwrap());
        }
        if data_id.is_some() {
            query.ne_(BmbpSettingDict::get_table_primary_key(), data_id.unwrap());
        }
        let dict_vec = BmbpRbacDictDao::select_list_by_query(&query).await?;
        match dict_vec {
            Some(dict_vec) => Ok(!dict_vec.is_empty()),
            None => Ok(false),
        }
    }

    pub async fn has_same_alias(alias: &String, data_id: Option<&String>) -> BmbpResp<bool> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_("dict_alias", alias);
        if data_id.is_some() {
            query.ne_(BmbpSettingDict::get_table_primary_key(), data_id.unwrap());
        }
        let dict_vec = BmbpRbacDictDao::select_list_by_query(&query).await?;
        match dict_vec {
            Some(dict_vec) => Ok(!dict_vec.is_empty()),
            None => Ok(false),
        }
    }

    pub async fn find_combo_by_alias(alias: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(v) = alias {
            if let Some(dict) = Self::find_dict_by_alias(&v).await? {
                let code = dict.get_code();
                combo = Self::find_combo_by_code(code).await?;
            }
        }
        Ok(combo)
    }

    pub async fn find_combo_by_code(code: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];

        if let Some(dict_vec) = Self::find_children_by_code(code).await? {
            combo = Self::convert_dict_list_to_combo(&dict_vec);
        }
        Ok(combo)
    }

    pub async fn find_combo_by_id(id: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(dict) = Self::find_dict_by_id(id).await? {
            let code = dict.get_code();
            combo = Self::find_combo_by_code(code).await?;
        }
        Ok(combo)
    }

    pub async fn find_multi_combo_by_alias(
        alias: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in alias {
            let dict = Self::find_combo_by_alias(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }

    pub async fn find_multi_combo_by_code(
        code: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in code {
            let dict = Self::find_combo_by_code(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }
    pub async fn find_multi_combo_by_id(
        id: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in id {
            let dict = Self::find_combo_by_id(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }

    pub async fn find_cascade_combo_by_alias(alias: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(v) = alias {
            if let Some(dict) = Self::find_dict_by_alias(&v).await? {
                let code_path = dict.get_code_path();
                combo = Self::find_cascade_combo_by_code_path(code_path).await?;
            }
        }
        Ok(combo)
    }

    pub async fn find_cascade_combo_by_code(code: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(v) = code {
            if let Some(dict) = Self::find_dict_by_code(&v).await? {
                let code_path = dict.get_code_path();
                combo = Self::find_cascade_combo_by_code_path(code_path).await?;
            }
        }
        Ok(combo)
    }

    pub async fn find_cascade_combo_by_id(id: Option<&String>) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(dict) = Self::find_dict_by_id(id).await? {
            let code_path = dict.get_code_path();
            combo = Self::find_cascade_combo_by_code_path(code_path).await?;
        }
        Ok(combo)
    }
    pub async fn find_cascade_combo_by_code_path(
        code_path: Option<&String>,
    ) -> BmbpResp<Vec<BmbpComboVo>> {
        let mut combo = vec![];
        if let Some(dict) = Self::find_children_by_code_path(code_path).await? {
            combo = Self::convert_dict_list_to_combo(&RdbcTreeUtil::build_tree(dict));
        }
        Ok(combo)
    }
    pub async fn find_multi_cascade_combo_by_alias(
        alias: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in alias {
            let dict = Self::find_cascade_combo_by_alias(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }
    pub async fn find_multi_cascade_combo_by_code(
        code: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in code {
            let dict = Self::find_cascade_combo_by_code(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }
    pub async fn find_multi_cascade_combo_by_id(
        id: Vec<String>,
    ) -> BmbpResp<HashMap<String, Vec<BmbpComboVo>>> {
        let mut mp = HashMap::new();
        for item in id {
            let dict = Self::find_cascade_combo_by_id(Some(&item)).await?;
            mp.insert(item, dict);
        }
        Ok(mp)
    }

    pub async fn find_translate_by_alias(
        alias: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(v) = alias {
            if let Some(dict) = Self::find_dict_by_alias(&v).await? {
                let code = dict.get_code();
                translate = Self::find_translate_by_code(code).await?;
            }
        }
        Ok(translate)
    }
    pub async fn find_translate_by_code(
        code: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();

        if let Some(dict) = Self::find_children_by_code(code).await? {
            translate = Self::convert_dict_list_to_translate(&dict);
        }

        Ok(translate)
    }
    pub async fn find_translate_by_id(id: Option<&String>) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(dict) = Self::find_dict_by_id(id).await? {
            let code = dict.get_code();
            translate = Self::find_translate_by_code(code).await?;
        }
        Ok(translate)
    }
    pub async fn find_multi_translate_by_alias(
        alias: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in alias {
            let dict = Self::find_translate_by_alias(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    pub async fn find_multi_translate_by_code(
        code: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in code {
            let dict = Self::find_translate_by_code(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    pub async fn find_multi_translate_by_id(
        id: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in id {
            let dict = Self::find_translate_by_id(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    pub async fn find_cascade_translate_by_alias(
        alias: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(v) = alias {
            if let Some(dict) = Self::find_dict_by_alias(&v).await? {
                let code_path = dict.get_code_path();
                translate = Self::find_cascade_translate_by_code_path(code_path).await?;
            }
        }
        Ok(translate)
    }
    pub async fn find_cascade_translate_by_code(
        code: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(v) = code {
            if let Some(dict) = Self::find_dict_by_code(&v).await? {
                let code_path = dict.get_code_path();
                translate = Self::find_cascade_translate_by_code_path(code_path).await?;
            }
        }
        Ok(translate)
    }
    pub async fn find_cascade_translate_by_id(
        id: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(dict) = Self::find_dict_by_id(id).await? {
            let code_path = dict.get_code_path();
            translate = Self::find_cascade_translate_by_code_path(code_path).await?;
        }
        Ok(translate)
    }
    pub async fn find_cascade_translate_by_code_path(
        code_path: Option<&String>,
    ) -> BmbpResp<HashMap<String, String>> {
        let mut translate = HashMap::new();
        if let Some(dict) = Self::find_children_by_code_path(code_path).await? {
            translate = Self::convert_dict_list_to_translate(&RdbcTreeUtil::build_tree(dict));
        }
        Ok(translate)
    }

    pub async fn find_multi_cascade_translate_by_alias(
        alias: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in alias {
            let dict = Self::find_cascade_translate_by_alias(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    pub async fn find_multi_cascade_translate_by_code(
        code: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in code {
            let dict = Self::find_cascade_translate_by_code(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    pub async fn find_multi_cascade_translate_by_id(
        id: Vec<String>,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        let mut translate = HashMap::new();
        for item in id {
            let dict = Self::find_cascade_translate_by_id(Some(&item)).await?;
            translate.insert(item, dict);
        }
        Ok(translate)
    }
    fn convert_dict_list_to_combo(mut dict_vec: &Vec<BmbpSettingDictOrmModel>) -> Vec<BmbpComboVo> {
        let mut como_vec = vec![];
        for dict in dict_vec {
            let mut vo = BmbpComboVo::new();
            if let Some(value) = dict.get_ext_props().get_dict_value() {
                vo.set_value(value.to_string());
            }
            if let Some(label) = dict.get_name() {
                vo.set_label(label.to_string());
            }
            let children = Self::convert_dict_list_to_combo(dict.get_children());
            vo.set_children(children);
            como_vec.push(vo);
        }
        como_vec
    }
    fn convert_dict_list_to_translate(
        mut dict_vec: &Vec<BmbpSettingDictOrmModel>,
    ) -> HashMap<String, String> {
        let mut mp = HashMap::new();
        for dict in dict_vec {
            let code = dict.get_ext_props().get_dict_value().unwrap();
            let name = dict.get_name().unwrap();
            mp.insert(code.to_string(), name.to_string());
            let child_mp = Self::convert_dict_list_to_translate(dict.get_children());
            for (k, v) in child_mp {
                let ck = format!("{}/{}", code, k);
                let cv = format!("{}/{}", name, v);
                mp.insert(ck, cv);
            }
        }
        mp
    }
    fn convert_dict_list_to_translate_nest(
        mut dict_vec: &Vec<BmbpSettingDictOrmModel>,
    ) -> Map<String, Value> {
        let mut mp = Map::new();
        for dict in dict_vec {
            let code = dict.get_ext_props().get_dict_value().unwrap();
            let name = dict.get_name().unwrap();
            mp.insert(code.to_string(), Value::String(name.to_string()));
            let child_mp = Self::convert_dict_list_to_translate_nest(dict.get_children());
            mp.insert("children".to_string(), Value::Object(child_mp));
        }
        mp
    }
}
