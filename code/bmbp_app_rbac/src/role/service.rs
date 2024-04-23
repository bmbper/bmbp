use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_utils::{is_empty_string, simple_uuid_upper};
use bmbp_rdbc_orm::{RDBC_DISABLE, RDBC_ENABLE, RdbcFilter, RdbcModel};

use crate::role::dao::BmbpRbacRoleDao;
use crate::role::model::{BmbpRbacRoleModel, RbacRoleQueryParams};
use crate::role::script::BmbpRbacRoleScript;

pub struct BmbpRbacRoleService;
impl BmbpRbacRoleService {
    pub async fn find_role_page(
        params: BmbpPageParam<RbacRoleQueryParams>,
    ) -> BmbpResp<PageVo<BmbpRbacRoleModel>> {
        // 拼接查询条件
        let mut query = BmbpRbacRoleScript::build_query_script();
        if let Some(role_params) = params.get_params() {
            if let Some(role_name) = role_params.get_role_name() {
                query.like("role_name", role_name);
            }
        };
        query.order_by("data_update_time", false);
        BmbpRbacRoleDao::select_page_by_query(params.get_page_no(), params.get_page_size(), &query)
            .await
    }
    pub async fn find_role_list(
        params: RbacRoleQueryParams,
    ) -> BmbpResp<Option<Vec<BmbpRbacRoleModel>>> {
        let mut query = BmbpRbacRoleScript::build_query_script();
        if let Some(role_name) = params.get_role_name() {
            query.like("role_name", role_name);
        }
        BmbpRbacRoleDao::select_list_by_query(&query).await
    }

    pub async fn find_role_by_id(id: Option<&String>) -> BmbpResp<Option<BmbpRbacRoleModel>> {
        if is_empty_string(id.as_ref()) {
            return Ok(None);
        }
        let mut query = BmbpRbacRoleScript::build_query_script();
        query.eq_(BmbpRbacRoleModel::get_table_primary_key(), id.unwrap());
        BmbpRbacRoleDao::select_one_by_query(&query).await
    }
    pub async fn insert_role(role: &mut BmbpRbacRoleModel) -> BmbpResp<usize> {
        // 设置公共默认值
        role.init_values();
        role.set_data_status("1".to_string());
        if is_empty_string(role.get_ext_props().get_role_name()) {
            return Err(BmbpError::service("角色名称不能为空"));
        }
        if is_empty_string(role.get_ext_props().get_role_code()) {
            role.get_mut_ext_props().set_role_code(simple_uuid_upper());
        }
        if Self::has_same_code(
            role.get_ext_props().get_role_code().unwrap(),
            role.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("角色编码已被占用，请修改后重试"));
        }
        if Self::has_same_name(
            role.get_ext_props().get_role_name().unwrap(),
            role.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("角色名称已被占用，请修改后重试"));
        }
        let insert = BmbpRbacRoleScript::build_insert(&role);
        BmbpRbacRoleDao::execute_insert(&insert).await
    }
    pub async fn update_role(role: &mut BmbpRbacRoleModel) -> BmbpResp<usize> {
        let old_organ_op = Self::find_role_by_id(role.get_data_id()).await?;
        if old_organ_op.is_none() {
            return Err(BmbpError::service("指定的角色不存在!"));
        }
        let role_old = old_organ_op.unwrap();
        {
            if is_empty_string(role.get_ext_props().get_role_name()) {
                if let Some(old_name) = role_old.get_ext_props().get_role_name() {
                    role.get_mut_ext_props().set_role_name(old_name.to_string());
                }
            }
            if is_empty_string(role.get_ext_props().get_role_code()) {
                if let Some(old_code) = role_old.get_ext_props().get_role_code() {
                    role.get_mut_ext_props().set_role_code(old_code.to_string());
                }
            }
            if is_empty_string(role.get_ext_props().get_role_desc()) {
                if let Some(old_desc) = role_old.get_ext_props().get_role_desc() {
                    role.get_mut_ext_props().set_role_desc(old_desc.to_string());
                }
            }
            if !role.get_data_remark().is_none() {
                if let Some(old_data_remark) = role_old.get_data_remark() {
                    role.set_data_remark(old_data_remark.to_string());
                }
            }
            if role.get_data_sort().is_none() {
                if let Some(old_organ_sort) = role_old.get_data_sort() {
                    role.set_data_sort(old_organ_sort.clone());
                }
            }
        }

        if is_empty_string(role.get_ext_props().get_role_name()) {
            return Err(BmbpError::service("角色名称不能为空"));
        }
        if is_empty_string(role.get_ext_props().get_role_code()) {
            role.get_mut_ext_props().set_role_code(simple_uuid_upper());
        }
        if Self::has_same_code(
            role.get_ext_props().get_role_code().unwrap(),
            role.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("角色编码已被占用，请修改后重试"));
        }
        if Self::has_same_name(
            role.get_ext_props().get_role_name().unwrap(),
            role.get_data_id(),
        )
        .await?
        {
            return Err(BmbpError::service("角色名称已被占用，请修改后重试"));
        }
        let update = BmbpRbacRoleScript::build_update(role);
        BmbpRbacRoleDao::execute_update(&update).await
    }
    pub async fn disable_role(role_id: Option<String>) -> BmbpResp<usize> {
        let role = Self::find_role_by_id(role_id.as_ref()).await?;
        if role.is_none() {
            return Err(BmbpError::service("待停用的角色不存在!"));
        }
        let update =
            BmbpRbacRoleScript::build_update_status(role_id.as_ref().unwrap(), RDBC_DISABLE);
        BmbpRbacRoleDao::execute_update(&update).await
    }

    pub async fn enable_role(role_id: Option<String>) -> BmbpResp<usize> {
        let role = Self::find_role_by_id(role_id.as_ref()).await?;
        if role.is_none() {
            return Err(BmbpError::service("待启用的角色不存在!"));
        }
        let update =
            BmbpRbacRoleScript::build_update_status(role_id.as_ref().unwrap(), RDBC_ENABLE);
        BmbpRbacRoleDao::execute_update(&update).await
    }

    pub async fn delete_role_by_id(role_id: Option<String>) -> BmbpResp<usize> {
        if is_empty_string(role_id.as_ref()) {
            return Err(BmbpError::service("请指定待删除的角色!"));
        }
        let delete_organ = BmbpRbacRoleScript::build_delete_script(role_id);
        BmbpRbacRoleDao::execute_delete(&delete_organ).await
    }

    pub async fn has_same_name(name: &String, data_id: Option<&String>) -> BmbpResp<bool> {
        let mut query = BmbpRbacRoleScript::build_query_script();
        query.eq_("role_name", name);
        if data_id.is_some() {
            query.ne_(BmbpRbacRoleModel::get_table_primary_key(), data_id.unwrap());
        }
        let model_vec = BmbpRbacRoleDao::select_list_by_query(&query).await?;
        match model_vec {
            Some(row_vec) => Ok(!row_vec.is_empty()),
            None => Ok(false),
        }
    }
    pub async fn has_same_code(code: &String, data_id: Option<&String>) -> BmbpResp<bool> {
        let mut query = BmbpRbacRoleScript::build_query_script();
        query.eq_("role_code", code);
        if data_id.is_some() {
            query.ne_(BmbpRbacRoleModel::get_table_primary_key(), data_id.unwrap());
        }
        let model_vec_op = BmbpRbacRoleDao::select_list_by_query(&query).await?;
        match model_vec_op {
            Some(mode_vec) => Ok(!mode_vec.is_empty()),
            None => Ok(false),
        }
    }
}
