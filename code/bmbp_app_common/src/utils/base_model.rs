use crate::{BmbpAuthInfo, BmbpBaseModel};
use chrono::Utc;
use uuid::Uuid;
/// 设置实体模型新增时，默认数据
pub fn set_insert_mode(base_mode: &mut BmbpBaseModel, auth: &BmbpAuthInfo) {
    if base_mode.get_record_id().is_none() || base_mode.get_record_id().unwrap().is_empty() {
        base_mode.set_record_id(Uuid::new_v4().to_string().replace("-", "").to_uppercase());
    }
    if base_mode.get_record_flag().is_none() || base_mode.get_record_flag().unwrap().is_empty() {
        base_mode.set_record_flag("0".to_string());
    }
    if base_mode.get_record_level().is_none() || base_mode.get_record_level().unwrap().is_empty() {
        base_mode.set_record_level("0".to_string());
    }
    if base_mode.get_record_status().is_none() || base_mode.get_record_status().unwrap().is_empty()
    {
        base_mode.set_record_status("0".to_string());
    }
    if base_mode.get_record_owner_user().is_none()
        || base_mode.get_record_owner_user().unwrap().is_empty()
    {
        base_mode.set_record_owner_user(auth.get_user_id().to_string());
    }

    if base_mode.get_record_owner_org().is_none()
        || base_mode.get_record_owner_org().unwrap().is_empty()
    {
        base_mode.set_record_owner_org(auth.get_user_org_code().to_string());
    }
    base_mode.set_record_create_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    base_mode.set_record_create_user(auth.get_user_id().to_string());
    base_mode.set_record_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    base_mode.set_record_update_user(auth.get_user_id().to_string());
}
/// 设置实体模型更新时，默认数据
pub fn set_update_mode(base_mode: &mut BmbpBaseModel, auth: &BmbpAuthInfo) {
    base_mode.set_record_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    base_mode.set_record_update_user(auth.get_user_id().to_string());
}
