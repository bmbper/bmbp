use async_trait::async_trait;
use bmbp_app_common::{BmbpHashMap, BmbpResp};

#[async_trait]
pub trait AuthService {
    /// 根据用户名称查询用户信息
    fn get_user_by_user_name() -> BmbpResp<Option<BmbpHashMap>> {
        Ok(None)
    }

    /// 根据用户名称查询用户所属组织信息
    fn get_organ_by_user_name() -> BmbpResp<Option<BmbpHashMap>> {
        Ok(None)
    }
    /// 根据用户名称查询用户角色信息
    fn get_role_by_user_name() -> BmbpResp<Vec<BmbpHashMap>> {
        Ok(vec![])
    }
    /// 根据用户名称查询用户应用信息
    fn get_app_by_user_name() -> BmbpResp<Vec<BmbpHashMap>> {
        Ok(vec![])
    }
    /// 根据用户名称查询用户应用菜单信息
    fn get_app_menu_by_user_name() -> BmbpResp<Vec<BmbpHashMap>> {
        Ok(vec![])
    }
    /// 根据用户名称查询用户接口信息
    fn get_app_url_by_user_name() -> BmbpResp<Vec<BmbpHashMap>> {
        Ok(vec![])
    }
    /// 根据用户名称查询用户数据权限信息
    fn get_app_data_scope_by_user_name() -> BmbpResp<Vec<BmbpHashMap>> {
        Ok(vec![])
    }
}
