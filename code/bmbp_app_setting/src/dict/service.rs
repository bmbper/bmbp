use tracing::info;

use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_utils::{is_empty_string, simple_uuid_upper};
use bmbp_rdbc_orm::{
    RDBC_DISABLE, RDBC_ENABLE, RDBC_TREE_ROOT_NODE, RdbcColumn, RdbcFilter, RdbcModel, RdbcTable,
    RdbcTableInner, RdbcTree, RdbcTreeUtil, simple_column, Update, value_column,
};

use crate::dict::dao::BmbpRbacDictDao;
use crate::dict::model::{BmbpDictType, BmbpSettingDict, BmbpSettingDictOrmModel, DictQueryParams};
use crate::dict::scripts::BmbpRdbcDictScript;

pub struct BmbpRbacDictService {}

impl BmbpRbacDictService {
    pub async fn query_dict_tree(
        params: DictQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        if let Some(dict_list) = Self::query_dict_list(params).await? {
            let dict_tree = RdbcTreeUtil::build_tree(dict_list);
            return Ok(Some(dict_tree));
        }
        Ok(None)
    }

    pub async fn query_dict_list(
        _params: DictQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let query = BmbpRdbcDictScript::build_query_script();
        BmbpRbacDictDao::select_list_by_query(&query).await
    }

    pub async fn query_dict_page(
        params: BmbpPageParam<DictQueryParams>,
    ) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
        // 拼接查询条件
        let query = BmbpRdbcDictScript::build_query_script();
        BmbpRbacDictDao::select_page_by_query(params.get_page_no(), params.get_page_size(), &query)
            .await
    }

    pub async fn query_dict_by_id(
        id: Option<&String>,
    ) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
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

    pub async fn insert_dict_info(dict: &mut BmbpSettingDictOrmModel) -> BmbpResp<usize> {
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
        if let Some(dict_parent) = Self::query_dict_by_code(dict.get_parent_code().unwrap()).await?
        {
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
        let insert = BmbpRdbcDictScript::build_insert(&dict);
        BmbpRbacDictDao::execute_insert(&insert).await
    }

    async fn query_dict_by_code(code: &String) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq_("code", code);
        BmbpRbacDictDao::select_one_by_query(&query).await
    }

    pub async fn update_dict_info(dict: &mut BmbpSettingDictOrmModel) -> BmbpResp<usize> {
        let old_dict_op = Self::query_dict_by_id(dict.get_data_id()).await?;
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
        let parent_dict_op = Self::query_dict_by_code(parent_dict_code.unwrap()).await?;
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

        let update = BmbpRdbcDictScript::build_update(dict);
        let mut row_count = BmbpRbacDictDao::execute_update(&update).await?;
        // 更新子表数据
        row_count =
            row_count + Self::update_name_path_for_children(dict.get_code_path().unwrap()).await?;

        Ok(row_count)
    }

    async fn update_name_path_for_children(code_path: &str) -> BmbpResp<usize> {
        // UPDATE damp_base_dict AS t1
        // JOIN damp_base_dict AS t2 ON t2.CODE = t1.parent_code
        // SET t1.NAME_PATH = CONCAT(t2.NAME_PATH, t1.name, '/')
        // WHERE t1.CODE_PATH LIKE '/9bd807b3c9db4aa6891e2fa4b099094e/95164d9eed66476387a8417154843672/%';
        let mut update = Update::new();
        update.table_alias(BmbpSettingDict::get_table_name(), "t1".to_string());
        let mut join_table =
            RdbcTableInner::table_alias(BmbpSettingDict::get_table_name(), "t2".to_string());
        join_table.on_eq("t2", "parent_code", "t1", "code");
        update.join_rdbc_table(join_table);
        let concat_column = RdbcColumn::concat(vec![
            simple_column("t2", "name_path"),
            simple_column("t1", "name"),
            value_column("/"),
        ]);
        update.set(simple_column("t1", "name_path"), concat_column);
        update.like_left_col(simple_column("t1", "code_path"), code_path);
        BmbpRbacDictDao::execute_update(&update).await
    }
    async fn update_code_path_for_children(parent_code_path: &str) -> BmbpResp<usize> {
        // UPDATE damp_base_dict AS t1
        // JOIN damp_base_dict AS t2 ON t2.CODE = t1.parent_code
        // SET t1.CODE_PATH = CONCAT(t2.CODE_PATH, t1.code, '/')
        // WHERE t1.CODE_PATH LIKE '/9bd807b3c9db4aa6891e2fa4b099094e/95164d9eed66476387a8417154843672/%';
        let mut update = Update::new();
        update.table_alias(BmbpSettingDict::get_table_name(), "t1".to_string());
        let mut join_table =
            RdbcTableInner::table_alias(BmbpSettingDict::get_table_name(), "t2".to_string());
        join_table.on_eq("t2", "parent_code", "t1", "code");
        update.join_rdbc_table(join_table);
        let concat_column = RdbcColumn::concat(vec![
            simple_column("t2", "code_path"),
            simple_column("t1", "code"),
            value_column("/"),
        ]);
        update.set(simple_column("t1", "code_path"), concat_column);
        update.like_left_col(simple_column("t1", "code_path"), parent_code_path);
        BmbpRbacDictDao::execute_update(&update).await
    }
    pub async fn disable_dict_status(dict_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(dict_id.as_ref()) {
            return Err(BmbpError::service("请指定待停用的字典!"));
        }
        let update = BmbpRdbcDictScript::build_update_status(dict_id, RDBC_ENABLE);
        BmbpRbacDictDao::execute_update(&update).await
    }

    pub async fn enable_dict_status(dict_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(dict_id.as_ref()) {
            return Err(BmbpError::service("请指定待启用的字典!"));
        }
        let update = BmbpRdbcDictScript::build_update_status(dict_id, RDBC_DISABLE);
        BmbpRbacDictDao::execute_update(&update).await
    }

    pub async fn delete_dict_info(dict_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(dict_id.as_ref()) {
            return Err(BmbpError::service("请指定待删除的字典!"));
        }
        let delete_dict = BmbpRdbcDictScript::build_delete_script(dict_id);
        BmbpRbacDictDao::execute_delete(&delete_dict).await
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
        let mut query = BmbpRdbcDictScript::build_query_script();
        if !dict_id.is_none() {
            query.not_like_left("code_path", dict_id);
        }

        BmbpRbacDictDao::select_list_by_query(&query).await
    }

    pub async fn update_dict_parent(
        _dict_id: Option<String>,
        _parent_code: Option<String>,
    ) -> BmbpResp<usize> {
        Ok(0)
    }
}
