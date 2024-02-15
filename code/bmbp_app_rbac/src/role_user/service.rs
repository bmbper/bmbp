use bmbp_app_common::BmbpError;
use bmbp_app_common::BmbpHashMap;
use bmbp_app_common::BmbpResp;
use bmbp_app_common::BmbpValue;
use bmbp_app_common::BmbpPageParam;
use bmbp_app_common::PageVo;
use bmbp_app_curd::CurdDao;
use bmbp_app_curd::CurdScript;
use bmbp_app_utils::add_insert_default_value;
use bmbp_app_utils::is_empty_prop;

use super::dao::RoleUserDao;
use super::script::RbacRoleUserScript;

pub struct RbacRoleUserService;
impl RbacRoleUserService {
    pub(crate) async fn find_role_tree_checked(params: &BmbpHashMap) -> BmbpResp<Vec<BmbpValue>> {
        if is_empty_prop(params, "userId") {
            return Err(BmbpError::service("请指定要查询的用户"));
        }
        let script = RbacRoleUserScript::query_role_checked_script();
        let mut row_checked_data = vec![];
        if let Some(role_user_data) = RoleUserDao::find_list(&script.to_script(), params).await? {
            for item in role_user_data.as_slice() {
                let v = item.get("roleId").unwrap().clone();
                row_checked_data.push(v);
            }
        }
        Ok(row_checked_data)
    }

    pub(crate) async fn find_role_page(
        page_params: &BmbpPageParam<BmbpHashMap>,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let mut script_params = BmbpHashMap::new();
        if let Some(params) = page_params.get_params() {
            if is_empty_prop(params, "userId") {
                return Err(BmbpError::service("请指定要查询的用户"));
            }
            script_params.insert("userId".to_string(), params.get("userId").unwrap().clone());
        }
        let script = RbacRoleUserScript::query_role_by_user_script();
        RoleUserDao::find_page(
            &script.to_script(),
            &script_params,
            page_params.get_page_no(),
            page_params.get_page_size(),
        )
        .await
    }

    pub(crate) async fn remove_role_by_id(record_id: &String) -> BmbpResp<usize> {
        let script = RbacRoleUserScript::delete_script_by_id();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        RoleUserDao::delete(&script.to_script(), &script_params).await
    }

    pub(crate) async fn remove_role_by_user(user_id: &String) -> BmbpResp<usize> {
        let script = RbacRoleUserScript::delete_script_by_user();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("userId".to_string(), BmbpValue::from(user_id));
        RoleUserDao::delete(&script.to_script(), &script_params).await
    }

    pub(crate) async fn add_user_role(params: &BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "userId") {
            return Err(BmbpError::service("请指分配的用户"));
        }
        if is_empty_prop(params, "roleIds") {
            return Err(BmbpError::service("请指分配的角色"));
        }
        let user_id = params.get("userId").unwrap().to_string();
        // 删除原有数据
        let mut row_count = Self::remove_role_by_user(&user_id).await?;
        tracing::info!("remove_role_count:{}", row_count);
        let mut user_role_vec = vec![];
        let role_ids_value = params.get("roleIds").unwrap();
        match role_ids_value {
            BmbpValue::Array(roles) => {
                if roles.len() > 0 {
                    for role in roles.as_slice() {
                        let mut temp_map = BmbpHashMap::new();
                        temp_map.insert("roleId".to_string(), role.clone());
                        temp_map.insert("userId".to_string(), BmbpValue::from(user_id.clone()));
                        user_role_vec.push(temp_map);
                    }
                }
            }
            _ => {
                return Err(BmbpError::service("分配的角色以数组的形式传入"));
            }
        }
        let mut_user_role_slice = user_role_vec.as_mut_slice();
        for item in mut_user_role_slice {
            let count = Self::insert(item).await?;
            row_count = row_count + count
        }
        Ok(row_count)
    }

    /// 插入记录
    pub(crate) async fn insert(user_role: &mut BmbpHashMap) -> BmbpResp<usize> {
        add_insert_default_value(user_role);
        let insert_script = RbacRoleUserScript::insert_script();
        RoleUserDao::insert(&insert_script.to_script(), user_role).await
    }
}
