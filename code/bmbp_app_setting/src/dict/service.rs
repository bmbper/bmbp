use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_utils::is_empty_string;
use bmbp_rdbc_orm::{RDBC_DISABLE, RDBC_ENABLE, RdbcModel};
use crate::dict::dao::{BmbpRbacDictDao};
use crate::dict::model::{BmbpSettingDict, BmbpSettingDictOrmModel, DictQueryParams};
use crate::dict::scripts::BmbpRdbcDictScript;

pub struct BmbpRbacDictService {}

impl BmbpRbacDictService {
    pub async fn query_dict_tree(params: DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        if let Some(dict_list) = Self::query_dict_list(params).await? {
            return Ok(Some(dict_list));
        }
        Ok(None)
    }

    pub async fn query_dict_list(_params: DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        let query = BmbpRdbcDictScript::build_query_script();
        BmbpRbacDictDao::select_list_by_query(&query).await
    }

    pub async fn query_dict_page(params: BmbpPageParam<DictQueryParams>) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
        // 拼接查询条件
        let query = BmbpRdbcDictScript::build_query_script();
        BmbpRbacDictDao::select_page_by_query(params.get_page_no(), params.get_page_size(), &query).await
    }


    pub async fn query_dict_by_id(id: Option<String>) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
        if is_empty_string(id.as_ref()) {
            return Ok(None);
        }
        let mut query = BmbpRdbcDictScript::build_query_script();
        query.eq(BmbpSettingDict::get_table_primary_key(), id.unwrap());
        BmbpRbacDictDao::select_one_by_query(&query).await
    }

    pub async fn insert_dict_info(dict: BmbpSettingDictOrmModel) -> BmbpResp<usize> {
        let insert = BmbpRdbcDictScript::build_insert(&dict);
        BmbpRbacDictDao::execute_insert(&insert).await
    }

    pub async fn update_dict_info(dict: BmbpSettingDictOrmModel) -> BmbpResp<usize> {
        let  update = BmbpRdbcDictScript::build_update(&dict);
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

    pub async fn query_dict_tree_exclude_by_id(_dict_id: Option<String>) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
        Ok(None)
    }

    pub async fn update_dict_parent(_dict_id: Option<String>, _parent_code: Option<String>) -> BmbpResp<usize> {
        Ok(0)
    }
}

